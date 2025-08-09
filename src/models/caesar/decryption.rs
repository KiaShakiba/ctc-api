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
#[diesel(table_name = schema::caesar_decryptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CaesarDecryption {
	id: i32,
	pub user_id: i32,

	key: i32,
	message: Option<String>,
	cipher: String,

	created_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
}


#[derive(Insertable)]
#[diesel(table_name = schema::caesar_decryptions)]
struct NewCaesarDecryption {
	user_id: i32,

	key: i32,
	cipher: String,
}

#[derive(Serialize)]
pub struct CaesarDecryptionPublic {
	key: i32,
	cipher: String,
}

impl CaesarDecryption {
	pub fn completed_duration(&self) -> Option<Duration> {
		let delta = self.completed_at?.signed_duration_since(self.created_at);
		let nanoseconds = delta.num_nanoseconds()? as u64;

		Some(Duration::from_nanos(nanoseconds))
	}

	pub async fn find_user_incomplete(
		state: &AppState,
		user_id: i32,
	) -> Result<Option<Self>, Error> {
		if let Some(cached_incomplete) = CaesarDecryption::from_cached(state.cache(), user_id)? {
			return Ok(Some(cached_incomplete));
		}

		let mut db = state.db().await?;

		let maybe_got = schema::caesar_decryptions::dsl::caesar_decryptions
			.filter(schema::caesar_decryptions::user_id.eq(user_id))
			.filter(schema::caesar_decryptions::message.is_null())
			.select(CaesarDecryption::as_select())
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

		let cipher_size = env::var("CAESAR_DECRYPTION_CIPHER_SIZE").ok()
			.and_then(|value| value.parse::<usize>().ok())
			.unwrap_or(6);

		let cipher = Alphabetic.sample_string(
			&mut rand::rng(),
			cipher_size,
		).to_uppercase();

		let new_decryption = NewCaesarDecryption {
			user_id,

			key,
			cipher,
		};

		let mut db = state.db().await?;

		let decryption = diesel::insert_into(schema::caesar_decryptions::table)
			.values(&new_decryption)
			.returning(CaesarDecryption::as_returning())
			.get_result(&mut db).await?;

		decryption.to_cached(state.cache(), user_id)?;

		Ok(decryption)
	}

	pub async fn try_into_completed(self, state: &AppState, message: String) -> Result<Duration, Error> {
		let decrypted = self.cipher
			.chars()
			.map(|char| {
				let old_ascii_index = char as u8 - 65;

				let new_ascii_index = if old_ascii_index >= self.key as u8 {
					old_ascii_index - self.key as u8
				} else {
					old_ascii_index + 26 - self.key as u8
				} % 26;

				(new_ascii_index + 65) as char
			})
			.collect::<String>();

		if decrypted != message {
			let error = Error::default()
				.with_code(StatusCode::BAD_REQUEST)
				.with_message("Incorrect message.");

			return Err(error);
		}

		let mut db = state.db().await?;

		let completed = diesel::update(schema::caesar_decryptions::dsl::caesar_decryptions.find(self.id))
			.set((
				schema::caesar_decryptions::dsl::message.eq(message),
				schema::caesar_decryptions::dsl::completed_at.eq(diesel::dsl::now),
			))
			.get_result::<Self>(&mut db).await?;

		CaesarDecryption::purge_cache(state.cache(), self.user_id)?;

		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		Ok(duration)
	}

	pub async fn find_all_completed(state: &AppState) -> Result<Vec<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::caesar_decryptions::dsl::caesar_decryptions
			.filter(schema::caesar_decryptions::message.is_not_null())
			.select(CaesarDecryption::as_select())
			.load(&mut db).await?
			.into_iter()
			.collect();

		Ok(got)
	}
}

impl Cachable for CaesarDecryption {
	type Id = i32;

	fn cache_key(user_id: Self::Id) -> String {
		format!("caesar:decryption:{user_id}:incomplete")
	}
}

impl From<CaesarDecryption> for CaesarDecryptionPublic {
	fn from(decryption: CaesarDecryption) -> Self {
		CaesarDecryptionPublic {
			key: decryption.key,
			cipher: decryption.cipher,
		}
	}
}
