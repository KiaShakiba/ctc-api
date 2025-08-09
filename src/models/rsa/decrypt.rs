use std::{
	env,
	time::Duration,
};

use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use chrono::{DateTime, Utc};
use primal_sieve::Sieve;

use rand::{
	Rng,
	seq::IteratorRandom,
};

use crate::{
	schema,
	math,
	error::Error,
	state::{AppState, Cachable},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::rsa_decrypts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RsaDecrypt {
	id: i32,
	pub user_id: i32,

	n_p: i64,
	n_q: i64,
	e: i64,
	d: i64,
	c: i64,

	created_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
}


#[derive(Insertable)]
#[diesel(table_name = schema::rsa_decrypts)]
struct NewRsaDecrypt {
	user_id: i32,

	n_p: i64,
	n_q: i64,
	e: i64,
	d: i64,
	c: i64,
}

#[derive(Serialize)]
pub struct RsaDecryptPublic {
	p: i64,
	q: i64,
	e: i64,
	d: i64,
	c: i64,
}

impl RsaDecrypt {
	pub fn completed_duration(&self) -> Option<Duration> {
		let delta = self.completed_at?.signed_duration_since(self.created_at);
		let nanoseconds = delta.num_nanoseconds()? as u64;

		Some(Duration::from_nanos(nanoseconds))
	}

	pub async fn find_user_incomplete(
		state: &AppState,
		user_id: i32,
	) -> Result<Option<Self>, Error> {
		if let Some(cached_incomplete) = RsaDecrypt::from_cached(state.cache(), user_id)? {
			return Ok(Some(cached_incomplete));
		}

		let mut db = state.db().await?;

		let maybe_got = schema::rsa_decrypts::dsl::rsa_decrypts
			.filter(schema::rsa_decrypts::user_id.eq(user_id))
			.filter(schema::rsa_decrypts::completed_at.is_null())
			.select(RsaDecrypt::as_select())
			.load(&mut db).await?
			.into_iter()
			.next();

		if let Some(got) = &maybe_got {
			got.to_cached(state.cache(), user_id)?;
		}

		Ok(maybe_got)
	}

	pub async fn create(state: &AppState, user_id: i32) -> Result<Self, Error> {
		let pq_min = env::var("RSA_PQ_MIN").ok()
			.and_then(|value| value.parse::<usize>().ok())
			.unwrap_or(1_000);

		let pq_max = env::var("RSA_PQ_MAX").ok()
			.and_then(|value| value.parse::<usize>().ok())
			.unwrap_or(10_000);

		let sieve = Sieve::new(pq_max);

		let p = sieve
			.primes_from(pq_min)
			.take_while(|n| *n <= pq_max)
			.choose(&mut rand::rng())
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let mut q = sieve
			.primes_from(pq_min)
			.take_while(|n| *n <= pq_max)
			.choose(&mut rand::rng())
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		while p == q {
			q = sieve
				.primes_from(pq_min)
				.take_while(|n| *n <= pq_max)
				.choose(&mut rand::rng())
				.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
		}

		let n = p * q;
		let totient = (p - 1) * (q - 1);

		let mut e: u64;
		let mut maybe_d: Option<u64>;

		let (d, m) = {
			let mut rng = rand::rng();

			loop {
				e = rng.random_range(1..(totient - 1)) as u64;
				maybe_d = math::inverse_mod(e, totient);

				if math::gcd(e, totient) == 1 && maybe_d.is_some_and(|d| d != e) {
					break;
				}
			}

			let d = maybe_d.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

			let m_min = n / 2;
			let m_max = n - 1;
			let m = rng.random_range(m_min..=m_max) as u64;

			(d, m)
		};

		let c = math::power_mod(m, e, n);

		let new_decrypt = NewRsaDecrypt {
			user_id,

			n_p: p as i64,
			n_q: q as i64,
			e: e as i64,
			d: d as i64,
			c: c as i64,
		};

		let mut db = state.db().await?;

		let decrypt = diesel::insert_into(schema::rsa_decrypts::table)
			.values(&new_decrypt)
			.returning(RsaDecrypt::as_returning())
			.get_result(&mut db).await?;

		decrypt.to_cached(state.cache(), user_id)?;

		Ok(decrypt)
	}

	pub async fn try_into_completed(self, state: &AppState, m: u64) -> Result<Duration, Error> {
		if m != math::power_mod(self.c, self.d, self.n_p * self.n_q) {
			let error = Error::default()
				.with_code(StatusCode::BAD_REQUEST)
				.with_message("Incorrect message.");

			return Err(error);
		}

		let mut db = state.db().await?;

		let completed = diesel::update(schema::rsa_decrypts::dsl::rsa_decrypts.find(self.id))
			.set(schema::rsa_decrypts::dsl::completed_at.eq(diesel::dsl::now))
			.get_result::<Self>(&mut db).await?;

		RsaDecrypt::purge_cache(state.cache(), self.user_id)?;

		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		Ok(duration)
	}

	pub async fn find_all_completed(state: &AppState) -> Result<Vec<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::rsa_decrypts::dsl::rsa_decrypts
			.filter(schema::rsa_decrypts::completed_at.is_not_null())
			.select(RsaDecrypt::as_select())
			.load(&mut db).await?
			.into_iter()
			.collect();

		Ok(got)
	}
}

impl Cachable for RsaDecrypt {
	type Id = i32;

	fn cache_key(user_id: Self::Id) -> String {
		format!("rsa:decrypt:{user_id}:incomplete")
	}
}

impl From<RsaDecrypt> for RsaDecryptPublic {
	fn from(decrypt: RsaDecrypt) -> Self {
		RsaDecryptPublic {
			p: decrypt.n_p,
			q: decrypt.n_q,
			e: decrypt.e,
			d: decrypt.d,
			c: decrypt.c,
		}
	}
}
