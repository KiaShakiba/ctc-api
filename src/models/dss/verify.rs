use std::{
	env,
	time::Duration,
};

use num_traits::AsPrimitive;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use chrono::{DateTime, Utc};
use primal_sieve::Sieve;

use rand::{
	Rng,
	seq::{IndexedRandom, IteratorRandom},
};

use crate::{
	schema,
	math,
	error::Error,
	state::{AppState, Cacheable},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::dss_verifies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DssVerify {
	id: i32,
	pub user_id: i32,

	n_p: i64,
	n_q: i64,
	g: i64,
	h: String,
	pk: i64,
	m: i64,
	r: i64,
	s: i64,

	created_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
}


#[derive(Insertable)]
#[diesel(table_name = schema::dss_verifies)]
struct NewDssVerify {
	user_id: i32,

	n_p: i64,
	n_q: i64,
	g: i64,
	h: String,
	pk: i64,
	m: i64,
	r: i64,
	s: i64,
}

#[derive(Serialize)]
pub struct DssVerifyPublic {
	p: i64,
	q: i64,
	g: i64,
	h: String,
	pk: i64,
	m: i64,
	r: i64,
	s: i64,
}

impl DssVerify {
	pub fn completed_duration(&self) -> Option<Duration> {
		let delta = self.completed_at?.signed_duration_since(self.created_at);
		let nanoseconds = delta.num_nanoseconds()? as u64;

		Some(Duration::from_nanos(nanoseconds))
	}

	pub async fn find_user_incomplete(
		state: &AppState,
		user_id: i32,
	) -> Result<Option<Self>, Error> {
		if let Some(cached_incomplete) = DssVerify::from_cached(state.cache(), user_id)? {
			return Ok(Some(cached_incomplete));
		}

		let mut db = state.db().await?;

		let maybe_got = schema::dss_verifies::dsl::dss_verifies
			.filter(schema::dss_verifies::user_id.eq(user_id))
			.filter(schema::dss_verifies::completed_at.is_null())
			.select(DssVerify::as_select())
			.load(&mut db).await?
			.into_iter()
			.next();

		if let Some(got) = &maybe_got {
			got.to_cached(state.cache(), user_id)?;
		}

		Ok(maybe_got)
	}

	pub async fn create(state: &AppState, user_id: i32) -> Result<Self, Error> {
		let p_min = env::var("DSS_P_MIN").ok()
			.and_then(|value| value.parse::<usize>().ok())
			.unwrap_or(1_000);

		let p_max = env::var("DSS_P_MAX").ok()
			.and_then(|value| value.parse::<usize>().ok())
			.unwrap_or(10_000);

		let sieve = Sieve::new(p_max);

		let mut p = sieve
			.primes_from(p_min)
			.take_while(|n| *n <= p_max)
			.choose(&mut rand::rng())
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let mut q: u64;
		let mut g: u64;

		loop {
			let mut factors = math::prime_factors(p - 1);

			while factors.is_empty() {
				p = sieve
					.primes_from(p_min)
					.take_while(|n| *n <= p_max)
					.choose(&mut rand::rng())
					.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

				factors = math::prime_factors(p - 1);
			}

			q = factors.into_iter().last().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
			g = 2;

			while math::order(g, p).is_none_or(|order| order != q) && g <= p as u64 {
				g += 1;
			}

			if math::order(g, p).is_some_and(|order| order == q) {
				break;
			}
		}

		let h = get_random_h()?;
		let mut k: u64;
		let mut r: u64;

		loop {
			k = rand::rng().random_range(1..q);
			r = math::safe_mod(math::power_mod(g, k, p), q);

			if r > 0 {
				break;
			}
		}

		let sk = rand::rng().random_range(1..q);
		let pk = math::power_mod(g, sk, p);

		let m_min = p / 2;
		let m_max = p - 1;
		let mut m = rand::rng().random_range(m_min..=m_max);
		let mut digest = get_h_digest(&h, m, q)?;

		while digest == 0 {
			m = rand::rng().random_range(m_min..=m_max);
			digest = get_h_digest(&h, m, q)?;
		}

		let k_inv_q = math::inverse_mod(k, q).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
		let s = math::safe_mod((digest as i64 - sk as i64 * r as i64) * k_inv_q as i64, q);

		let new_verify = NewDssVerify {
			user_id,

			n_p: p as i64,
			n_q: q as i64,
			g: g as i64,
			h,
			pk: pk as i64,
			m: m as i64,
			r: r as i64,
			s: s as i64,
		};

		let mut db = state.db().await?;

		let verify = diesel::insert_into(schema::dss_verifies::table)
			.values(&new_verify)
			.returning(DssVerify::as_returning())
			.get_result(&mut db).await?;

		verify.to_cached(state.cache(), user_id)?;

		Ok(verify)
	}

	pub async fn try_into_completed(
		self,
		state: &AppState,
		u: u64,
		v: u64,
		w: u64,
	) -> Result<Duration, Error> {
		let error = Error::default()
			.with_code(StatusCode::BAD_REQUEST)
			.with_message("Incorrect signature.");

		if self.pk >= self.n_p || self.r >= self.n_q || self.s >= self.n_q {
			return Err(error);
		}

		let digest = get_h_digest(&self.h, self.m, self.n_q)?;
		let s_inv_q = math::inverse_mod(self.s, self.n_q).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let correct_u = math::safe_mod(digest * s_inv_q, self.n_q);
		let correct_v = math::safe_mod(math::safe_mod(-self.r, self.n_q) * s_inv_q, self.n_q);

		let correct_w = math::safe_mod(
			math::safe_mod(
				math::power_mod(self.g, u, self.n_p) * math::power_mod(self.pk, v, self.n_p),
				self.n_p,
			),
			self.n_q,
		);

		if u != correct_u || v != correct_v || w != correct_w || correct_w != self.r as u64 {
			return Err(error);
		}

		let mut db = state.db().await?;

		let completed = diesel::update(schema::dss_verifies::dsl::dss_verifies.find(self.id))
			.set(schema::dss_verifies::dsl::completed_at.eq(diesel::dsl::now))
			.get_result::<Self>(&mut db).await?;

		DssVerify::purge_cache(state.cache(), self.user_id)?;

		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		Ok(duration)
	}

	pub async fn find_all_completed(state: &AppState) -> Result<Vec<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::dss_verifies::dsl::dss_verifies
			.filter(schema::dss_verifies::completed_at.is_not_null())
			.select(DssVerify::as_select())
			.load(&mut db).await?
			.into_iter()
			.collect();

		Ok(got)
	}
}

fn get_random_h() -> Result<String, Error> {
	["m mod q", "2m mod q", "3m mod q"]
		.choose(&mut rand::rng())
		.map(|value| value.to_string())
		.ok_or(Error::default())
}

fn get_h_digest(
	h: &str,
	m: impl AsPrimitive<u64>,
	q: impl AsPrimitive<u64>,
) -> Result<u64, Error> {
	match h {
		"m mod q" => Ok(math::safe_mod(m.as_(), q.as_())),
		"2m mod q" => Ok(math::safe_mod(m.as_() * 2, q.as_())),
		"3m mod q" => Ok(math::safe_mod(m.as_() * 3, q.as_())),

		_ => Err(Error::default()),
	}
}

impl Cacheable for DssVerify {
	type Id = i32;

	fn cache_key(user_id: Self::Id) -> String {
		format!("rsa:verify:{user_id}:incomplete")
	}
}

impl From<DssVerify> for DssVerifyPublic {
	fn from(verify: DssVerify) -> Self {
		DssVerifyPublic {
			p: verify.n_p,
			q: verify.n_q,
			g: verify.g,
			h: verify.h,
			pk: verify.pk,
			m: verify.m,
			r: verify.r,
			s: verify.s,
		}
	}
}
