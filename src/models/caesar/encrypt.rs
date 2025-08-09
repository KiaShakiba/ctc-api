use std::{
	env,
	time::Duration,
};

use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use chrono::{DateTime, Utc};
use rand::distr::{Alphabetic, SampleString};

use crate::{
	schema,
	error::Error,
	state::{AppState, Cachable},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::caesar_encrypts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CaesarEncrypt {
	id: i32,
	pub user_id: i32,

	key: i32,
	message: String,
	cipher: Option<String>,

	created_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
}


#[derive(Insertable)]
#[diesel(table_name = schema::caesar_encrypts)]
struct NewCaesarEncrypt {
	user_id: i32,

	key: i32,
	message: String,
}

#[derive(Serialize)]
pub struct CaesarEncryptPublic {
	key: i32,
	message: String,
}

impl CaesarEncrypt {
	pub fn completed_duration(&self) -> Option<Duration> {
		let delta = self.completed_at?.signed_duration_since(self.created_at);
		let nanoseconds = delta.num_nanoseconds()? as u64;

		Some(Duration::from_nanos(nanoseconds))
	}

	pub async fn find_user_incomplete(
		state: &AppState,
		user_id: i32,
	) -> Result<Option<Self>, Error> {
		if let Some(cached_incomplete) = CaesarEncrypt::from_cached(state.cache(), user_id)? {
			return Ok(Some(cached_incomplete));
		}

		let mut db = state.db().await?;

		let maybe_got = schema::caesar_encrypts::dsl::caesar_encrypts
			.filter(schema::caesar_encrypts::user_id.eq(user_id))
			.filter(schema::caesar_encrypts::cipher.is_null())
			.select(CaesarEncrypt::as_select())
			.load(&mut db).await?
			.into_iter()
			.next();

		if let Some(got) = &maybe_got {
			got.to_cached(state.cache(), user_id)?;
		}

		Ok(maybe_got)
	}

	pub async fn create(state: &AppState, user_id: i32) -> Result<Self, Error> {
		let key = rand::random_range(8..=18);

		let message_size = env::var("CAESAR_ENCRYPTION_MESSAGE_SIZE").ok()
			.and_then(|value| value.parse::<usize>().ok())
			.unwrap_or(6);

		let message = Alphabetic.sample_string(
			&mut rand::rng(),
			message_size,
		).to_uppercase();

		let new_encrypt = NewCaesarEncrypt {
			user_id,

			key,
			message,
		};

		let mut db = state.db().await?;

		let encrypt = diesel::insert_into(schema::caesar_encrypts::table)
			.values(&new_encrypt)
			.returning(CaesarEncrypt::as_returning())
			.get_result(&mut db).await?;

		encrypt.to_cached(state.cache(), user_id)?;

		Ok(encrypt)
	}

	pub async fn try_into_completed(self, state: &AppState, cipher: String) -> Result<Duration, Error> {
		let encrypted = self.message
			.chars()
			.map(|char| {
				let old_ascii_index = char as u8 - 65;
				let new_ascii_index = (old_ascii_index + self.key as u8) % 26;

				(new_ascii_index + 65) as char
			})
			.collect::<String>();

		if encrypted != cipher {
			let error = Error::default()
				.with_code(StatusCode::BAD_REQUEST)
				.with_message("Incorrect cipher.");

			return Err(error);
		}

		let mut db = state.db().await?;

		let completed = diesel::update(schema::caesar_encrypts::dsl::caesar_encrypts.find(self.id))
			.set((
				schema::caesar_encrypts::dsl::cipher.eq(cipher),
				schema::caesar_encrypts::dsl::completed_at.eq(diesel::dsl::now),
			))
			.get_result::<Self>(&mut db).await?;

		CaesarEncrypt::purge_cache(state.cache(), self.user_id)?;

		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		Ok(duration)
	}

	pub async fn find_all_completed(state: &AppState) -> Result<Vec<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::caesar_encrypts::dsl::caesar_encrypts
			.filter(schema::caesar_encrypts::cipher.is_not_null())
			.select(CaesarEncrypt::as_select())
			.load(&mut db).await?
			.into_iter()
			.collect();

		Ok(got)
	}
}

impl Cachable for CaesarEncrypt {
	type Id = i32;

	fn cache_key(user_id: Self::Id) -> String {
		format!("caesar:encrypt:{user_id}:incomplete")
	}
}

impl From<CaesarEncrypt> for CaesarEncryptPublic {
	fn from(encrypt: CaesarEncrypt) -> Self {
		CaesarEncryptPublic {
			key: encrypt.key,
			message: encrypt.message,
		}
	}
}
