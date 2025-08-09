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
#[diesel(table_name = schema::caesar_attacks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CaesarAttack {
	id: i32,
	pub user_id: i32,

	key: Option<i32>,
	message: String,
	cipher: String,

	created_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
}


#[derive(Insertable)]
#[diesel(table_name = schema::caesar_attacks)]
struct NewCaesarAttack {
	user_id: i32,

	message: String,
	cipher: String,
}

#[derive(Serialize)]
pub struct CaesarAttackPublic {
	message: String,
	cipher: String,
}

impl CaesarAttack {
	pub fn completed_duration(&self) -> Option<Duration> {
		let delta = self.completed_at?.signed_duration_since(self.created_at);
		let nanoseconds = delta.num_nanoseconds()? as u64;

		Some(Duration::from_nanos(nanoseconds))
	}

	pub async fn find_user_incomplete(
		state: &AppState,
		user_id: i32,
	) -> Result<Option<Self>, Error> {
		if let Some(cached_incomplete) = CaesarAttack::from_cached(state.cache(), user_id)? {
			return Ok(Some(cached_incomplete));
		}

		let mut db = state.db().await?;

		let maybe_got = schema::caesar_attacks::dsl::caesar_attacks
			.filter(schema::caesar_attacks::user_id.eq(user_id))
			.filter(schema::caesar_attacks::completed_at.is_null())
			.select(CaesarAttack::as_select())
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

		let message_size = env::var("CAESAR_ATTACK_MESSAGE_SIZE").ok()
			.and_then(|value| value.parse::<usize>().ok())
			.unwrap_or(6);

		let message = Alphabetic.sample_string(
			&mut rand::rng(),
			message_size,
		).to_uppercase();

		let cipher = message
			.chars()
			.map(|char| {
				let old_ascii_index = char as u8 - 65;
				let new_ascii_index = (old_ascii_index + key as u8) % 26;

				(new_ascii_index + 65) as char
			})
			.collect::<String>();

		let new_attack = NewCaesarAttack {
			user_id,

			message,
			cipher,
		};

		let mut db = state.db().await?;

		let attack = diesel::insert_into(schema::caesar_attacks::table)
			.values(&new_attack)
			.returning(CaesarAttack::as_returning())
			.get_result(&mut db).await?;

		attack.to_cached(state.cache(), user_id)?;

		Ok(attack)
	}

	pub async fn try_into_completed(self, state: &AppState, key: i32) -> Result<Duration, Error> {
		let encrypted = self.message
			.chars()
			.map(|char| {
				let old_ascii_index = char as u8 - 65;
				let new_ascii_index = (old_ascii_index + key as u8) % 26;

				(new_ascii_index + 65) as char
			})
			.collect::<String>();

		if encrypted != self.cipher {
			let error = Error::default()
				.with_code(StatusCode::BAD_REQUEST)
				.with_message("Incorrect key.");

			return Err(error);
		}

		let mut db = state.db().await?;

		let completed = diesel::update(schema::caesar_attacks::dsl::caesar_attacks.find(self.id))
			.set((
				schema::caesar_attacks::dsl::key.eq(key),
				schema::caesar_attacks::dsl::completed_at.eq(diesel::dsl::now),
			))
			.get_result::<Self>(&mut db).await?;

		CaesarAttack::purge_cache(state.cache(), self.user_id)?;

		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		Ok(duration)
	}

	pub async fn find_all_completed(state: &AppState) -> Result<Vec<Self>, Error> {
		let mut db = state.db().await?;

		let got = schema::caesar_attacks::dsl::caesar_attacks
			.filter(schema::caesar_attacks::completed_at.is_not_null())
			.select(CaesarAttack::as_select())
			.load(&mut db).await?
			.into_iter()
			.collect();

		Ok(got)
	}
}

impl Cachable for CaesarAttack {
	type Id = i32;

	fn cache_key(user_id: Self::Id) -> String {
		format!("caesar:attack:{user_id}:incomplete")
	}
}

impl From<CaesarAttack> for CaesarAttackPublic {
	fn from(attack: CaesarAttack) -> Self {
		CaesarAttackPublic {
			message: attack.message,
			cipher: attack.cipher,
		}
	}
}
