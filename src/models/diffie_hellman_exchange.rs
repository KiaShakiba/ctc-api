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
#[diesel(table_name = schema::diffie_hellman_exchanges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DiffieHellmanExchange {
	id: i32,
	pub user_id: i32,

	g: i64,
	n: i64,
	sk_server: i64,

	created_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
}


#[derive(Insertable)]
#[diesel(table_name = schema::diffie_hellman_exchanges)]
struct NewDiffieHellmanExchange {
	user_id: i32,

	g: i64,
	n: i64,
	sk_server: i64,
}

#[derive(Serialize)]
pub struct DiffieHellmanExchangePublic {
	g: u64,
	n: u64,
	pk_server: u64,
}

impl DiffieHellmanExchange {
	pub fn completed_duration(&self) -> Option<Duration> {
		let delta = self.completed_at?.signed_duration_since(self.created_at);
		let nanoseconds = delta.num_nanoseconds()? as u64;

		Some(Duration::from_nanos(nanoseconds))
	}

	fn pk_server(&self) -> u64 {
		math::power_mod(self.g, self.sk_server, self.n)
	}

	pub async fn find_user_incomplete(
		state: &AppState,
		user_id: i32,
	) -> Result<Option<Self>, Error> {
		if let Some(cached_incomplete) = DiffieHellmanExchange::from_cached(state.cache(), user_id)? {
			return Ok(Some(cached_incomplete));
		}

		let mut db = state.db().await?;

		let maybe_got = schema::diffie_hellman_exchanges::dsl::diffie_hellman_exchanges
			.filter(schema::diffie_hellman_exchanges::user_id.eq(user_id))
			.filter(schema::diffie_hellman_exchanges::completed_at.is_null())
			.select(DiffieHellmanExchange::as_select())
			.load(&mut db).await?
			.into_iter()
			.next();

		if let Some(got) = &maybe_got {
			got.to_cached(state.cache(), user_id)?;
		}

		Ok(maybe_got)
	}

	pub async fn create(state: &AppState, user_id: i32) -> Result<Self, Error> {
		let n_min = env::var("DIFFIE_HELLMAN_N_MIN")?.parse::<usize>()?;
		let n_max = env::var("DIFFIE_HELLMAN_N_MAX")?.parse::<usize>()?;

		let n = Sieve::new(n_max)
			.primes_from(n_min)
			.take_while(|n| *n <= n_max)
			.choose(&mut rand::rng())
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let g = math::primitive(n)
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let sk_server_min = n / 2;
		let sk_server_max = n - 1;
		let sk_server = rand::rng().random_range(sk_server_min..=sk_server_max);

		let new_exchange = NewDiffieHellmanExchange {
			user_id,

			g: g as i64,
			n: n as i64,
			sk_server: sk_server as i64,
		};

		let mut db = state.db().await?;

		let exchange = diesel::insert_into(schema::diffie_hellman_exchanges::table)
			.values(&new_exchange)
			.returning(DiffieHellmanExchange::as_returning())
			.get_result(&mut db).await?;

		exchange.to_cached(state.cache(), user_id)?;

		Ok(exchange)
	}

	pub async fn try_into_completed(self, state: &AppState, pk_user: u64, k: u64) -> Result<Duration, Error> {
		let mut db = state.db().await?;

		if k != math::power_mod(pk_user, self.sk_server, self.n) {
			let error = Error::default()
				.with_code(StatusCode::BAD_REQUEST)
				.with_message("Incorrect derived key.");

			return Err(error);
		}

		let completed = diesel::update(schema::diffie_hellman_exchanges::dsl::diffie_hellman_exchanges.find(self.id))
			.set(schema::diffie_hellman_exchanges::dsl::completed_at.eq(diesel::dsl::now))
			.get_result::<Self>(&mut db).await?;

		DiffieHellmanExchange::purge_cache(state.cache(), self.user_id)?;

		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		Ok(duration)
	}

	pub async fn find_all_completed(state: &AppState) -> Result<Vec<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::diffie_hellman_exchanges::dsl::diffie_hellman_exchanges
			.filter(schema::diffie_hellman_exchanges::completed_at.is_not_null())
			.select(DiffieHellmanExchange::as_select())
			.load(&mut db).await?
			.into_iter()
			.collect();

		Ok(got)
	}
}

impl Cachable for DiffieHellmanExchange {
	type Id = i32;

	fn cache_key(user_id: Self::Id) -> String {
		format!("diffie-hellman-exchange:{user_id}:incomplete")
	}
}

impl From<DiffieHellmanExchange> for DiffieHellmanExchangePublic {
	fn from(exchange: DiffieHellmanExchange) -> Self {
		DiffieHellmanExchangePublic {
			g: exchange.g as u64,
			n: exchange.n as u64,
			pk_server: exchange.pk_server(),
		}
	}
}
