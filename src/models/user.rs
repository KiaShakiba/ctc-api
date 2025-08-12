use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rand::distr::{Alphanumeric, SampleString};
use postcard::{to_allocvec, from_bytes};

use crate::{
	schema,
	error::Error,
	state::{AppState, Cacheable, DEFAULT_TTL},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
	pub id: i32,

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
		let user_key = format!("user:{}:bearer", self.id);
		let mut cache = state.cache();

		if let Ok(bearer_token_bytes) = cache.get(&user_key)
			&& let Ok(bearer_token) = TryInto::<String>::try_into(bearer_token_bytes)
		{
			cache.ttl(user_key, DEFAULT_TTL)?;
			cache.ttl(format!("bearer:{bearer_token}"), DEFAULT_TTL)?;

			return Ok(bearer_token);
		}

		let new_bearer_token = Alphanumeric.sample_string(&mut rand::rng(), 32);
		let cache_bytes = to_allocvec(self)?;

		cache.set(user_key, &new_bearer_token, DEFAULT_TTL)?;
		cache.set(format!("bearer:{new_bearer_token}"), cache_bytes, DEFAULT_TTL)?;

		Ok(new_bearer_token)
	}

	pub async fn find_by_id(state: &AppState, id: i32) -> Result<Option<Self>, Error> {
		if let Some(cached) = User::from_cached(state.cache(), id)? {
			return Ok(Some(cached));
		}

		let mut db = state.db().await?;

		let maybe_user = schema::users::dsl::users
			.find(id)
			.select(User::as_select())
			.first(&mut db).await
			.optional()?;

		if let Some(user) = &maybe_user {
			user.to_cached(state.cache(), user.id)?;
		}

		Ok(maybe_user)
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
		let bearer_key = format!("bearer:{bearer_token}");
		let mut cache = state.cache();

		let Ok(cache_bytes) = cache.get(&bearer_key) else {
			return Ok(None);
		};

		let user: User = from_bytes((&cache_bytes).into())?;

		cache.ttl(format!("user:{}:bearer", user.id), DEFAULT_TTL)?;
		cache.ttl(bearer_key, DEFAULT_TTL)?;

		Ok(Some(user))
	}

	pub async fn create(state: &AppState, new_user: NewUser) -> Result<Self, Error> {
		let mut db = state.db().await?;

		let user = diesel::insert_into(schema::users::table)
			.values(&new_user)
			.returning(User::as_returning())
			.get_result(&mut db).await?;

		user.to_cached(state.cache(), user.id)?;

		Ok(user)
	}
}

impl Cacheable for User {
	type Id = i32;

	fn cache_key(id: Self::Id) -> String {
		format!("user:{id}")
	}
}
