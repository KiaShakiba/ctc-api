use serde::Deserialize;
use axum_valid::Valid;
use validator::Validate;
use once_cell::sync::Lazy;
use regex::Regex;

use axum::{
	Router,
	extract::{State, Json},
	routing::post,
	http::StatusCode,
	response::IntoResponse,
};

use argon2::{
	Argon2,
	password_hash::{
		PasswordHash,
		PasswordHasher,
		PasswordVerifier,
		SaltString,
		rand_core::OsRng,
	},
};

use crate::{
	error::Error,
	state::AppState,
	models::user::{User, NewUser},
};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"^[a-zA-Z0-9]*$").unwrap()
});

#[derive(Deserialize, Validate)]
struct CreateUserBody {
	#[validate(regex(path = *USERNAME_REGEX, message = "Username can only contain alphanumeric characters."))]
	#[validate(length(min = 2, max = 16, message = "Username must be between 2 and 16 characters."))]
	username: String,

	#[validate(length(min = 6, message = "Password must be at least 6 characters."))]
	password: String,
}

async fn create_user(
	State(state): State<AppState>,
	Valid(Json(body)): Valid<Json<CreateUserBody>>,
) -> Result<impl IntoResponse, Error> {
	if let Some(user) = User::find_by_username(&state, &body.username).await? {
		let argon2 = Argon2::default();

		let parsed_hash = PasswordHash::new(&user.password_hash)
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

		if argon2.verify_password(body.password.as_bytes(), &parsed_hash).is_err() {
			let error = Error::default()
				.with_code(StatusCode::FORBIDDEN)
				.with_message("Invalid password.");

			return Err(error);
		}

		return Ok((StatusCode::OK, user.init_bearer_token(&state)?));
	}

	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();

	let password_hash = argon2
		.hash_password(body.password.as_bytes(), &salt)
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.to_string();

	let new_user = NewUser {
		username: body.username,
		password_hash,
	};

	let user = User::create(&state, new_user).await?;
	let bearer_token = user.init_bearer_token(&state)?;

	Ok((StatusCode::CREATED, bearer_token))
}

pub fn guarded_router() -> Router<AppState> {
	Router::new()
}

pub fn unguarded_router() -> Router<AppState> {
	Router::new()
		.route("/", post(create_user))
}
