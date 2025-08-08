use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rand::distr::{Alphanumeric, SampleString};
use postcard::{to_allocvec, from_bytes};

use crate::{
	schema,
	error::Error,
	state::AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
	id: i32,

	pub username: String,
	pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
	pub username: String,
	pub password_hash: String,
}

impl User {
	pub fn init_bearer_token(&self, state: &AppState) -> Result<String, Error> {
		let bearer_token = Alphanumeric.sample_string(&mut rand::rng(), 32);

		let mut cache = state.cache();
		let cache_bytes = to_allocvec(self).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

		cache.set(
			format!("bearer:{bearer_token}"),
			cache_bytes,
			Some(3_600),
		)?;

		Ok(bearer_token)
	}

	pub async fn find_by_username(state: &AppState, username: &str) -> Result<Option<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::users::dsl::users
			.filter(schema::users::username.eq(username))
			.select(User::as_select())
			.load(&mut db).await?;

		Ok(got.into_iter().next())
	}

	pub async fn find_by_bearer(state: &AppState, bearer_token: &str) -> Result<Option<Self>, Error> {
		let mut cache = state.cache();

		let Ok(cache_bytes) = cache.get(format!("bearer:{bearer_token}")) else {
			return Ok(None);
		};

		let user: User = from_bytes((&cache_bytes).into()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
		Ok(Some(user))
	}

	pub async fn create(state: &AppState, new_user: NewUser) -> Result<Self, Error> {
		let mut db = state.db().await?;

		let user = diesel::insert_into(schema::users::table)
			.values(&new_user)
			.returning(User::as_returning())
			.get_result(&mut db).await?;

		Ok(user)
	}
}
