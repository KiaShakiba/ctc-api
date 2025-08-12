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
#[diesel(table_name = schema::dss_signs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DssSign {
	id: i32,
	pub user_id: i32,

	n_p: i64,
	n_q: i64,
	g: i64,
	h: String,
	m: i64,

	created_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
}


#[derive(Insertable)]
#[diesel(table_name = schema::dss_signs)]
struct NewDssSign {
	user_id: i32,

	n_p: i64,
	n_q: i64,
	g: i64,
	h: String,
	m: i64,
}

#[derive(Serialize)]
pub struct DssSignPublic {
	p: i64,
	q: i64,
	g: i64,
	h: String,
	m: i64,
}

impl DssSign {
	pub fn completed_duration(&self) -> Option<Duration> {
		let delta = self.completed_at?.signed_duration_since(self.created_at);
		let nanoseconds = delta.num_nanoseconds()? as u64;

		Some(Duration::from_nanos(nanoseconds))
	}

	pub async fn find_user_incomplete(
		state: &AppState,
		user_id: i32,
	) -> Result<Option<Self>, Error> {
		if let Some(cached_incomplete) = DssSign::from_cached(state.cache(), user_id)? {
			return Ok(Some(cached_incomplete));
		}

		let mut db = state.db().await?;

		let maybe_got = schema::dss_signs::dsl::dss_signs
			.filter(schema::dss_signs::user_id.eq(user_id))
			.filter(schema::dss_signs::completed_at.is_null())
			.select(DssSign::as_select())
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

		let m_min = p / 2;
		let m_max = p - 1;
		let mut m = rand::rng().random_range(m_min..=m_max);
		let mut digest = get_h_digest(&h, m, q)?;

		while digest == 0 {
			m = rand::rng().random_range(m_min..=m_max);
			digest = get_h_digest(&h, m, q)?;
		}

		let new_sign = NewDssSign {
			user_id,

			n_p: p as i64,
			n_q: q as i64,
			g: g as i64,
			h,
			m: m as i64,
		};

		let mut db = state.db().await?;

		let sign = diesel::insert_into(schema::dss_signs::table)
			.values(&new_sign)
			.returning(DssSign::as_returning())
			.get_result(&mut db).await?;

		sign.to_cached(state.cache(), user_id)?;

		Ok(sign)
	}

	pub async fn try_into_completed(
		self,
		state: &AppState,
		pk: u64,
		r: u64,
		s: u64,
	) -> Result<Duration, Error> {
		let error = Error::default()
			.with_code(StatusCode::BAD_REQUEST)
			.with_message("Incorrect signature.");

		if pk >= self.n_p as u64 || r >= self.n_q as u64 || s >= self.n_q as u64 {
			return Err(error);
		}

		let digest = get_h_digest(&self.h, self.m, self.n_q)?;
		let s_inv_q = math::inverse_mod(s, self.n_q).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let u = math::safe_mod(digest * s_inv_q, self.n_q);
		let v = math::safe_mod(math::safe_mod(-(r as i64), self.n_q) * s_inv_q, self.n_q);

		let w = math::safe_mod(
			math::safe_mod(
				math::power_mod(self.g, u, self.n_p) * math::power_mod(pk, v, self.n_p),
				self.n_p,
			),
			self.n_q,
		);

		if w != r {
			return Err(error);
		}

		let mut db = state.db().await?;

		let completed = diesel::update(schema::dss_signs::dsl::dss_signs.find(self.id))
			.set(schema::dss_signs::dsl::completed_at.eq(diesel::dsl::now))
			.get_result::<Self>(&mut db).await?;

		DssSign::purge_cache(state.cache(), self.user_id)?;

		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		Ok(duration)
	}

	pub async fn find_all_completed(state: &AppState) -> Result<Vec<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::dss_signs::dsl::dss_signs
			.filter(schema::dss_signs::completed_at.is_not_null())
			.select(DssSign::as_select())
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

impl Cacheable for DssSign {
	type Id = i32;

	fn cache_key(user_id: Self::Id) -> String {
		format!("rsa:sign:{user_id}:incomplete")
	}
}

impl From<DssSign> for DssSignPublic {
	fn from(sign: DssSign) -> Self {
		DssSignPublic {
			p: sign.n_p,
			q: sign.n_q,
			g: sign.g,
			h: sign.h,
			m: sign.m,
		}
	}
}
