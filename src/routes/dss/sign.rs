use serde::Deserialize;
use axum_valid::Valid;
use validator::Validate;

use axum::{
	Router,
	extract::{State, Json, Extension},
	routing::{get, post},
	http::StatusCode,
};

use crate::{
	error::Error,
	state::AppState,
	models::{
		user::User,
		dss::{DssSign, DssSignPublic},
	},
	leaderboard::{Leaderboard, LeaderboardResult},
};

#[derive(Deserialize, Validate)]
struct SubmitEncryptBody {
	pk: u64,
	r: u64,
	s: u64,
}

async fn create_sign(
	State(state): State<AppState>,
	Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<DssSignPublic>), Error> {
	if let Some(existing) = DssSign::find_user_incomplete(&state, user.id).await? {
		let public: DssSignPublic = existing.into();
		return Ok((StatusCode::OK, Json(public)));
	}

	let sign = DssSign::create(&state, user.id).await?;
	let public: DssSignPublic = sign.into();

	Ok((StatusCode::CREATED, Json(public)))
}

async fn submit_sign(
	State(state): State<AppState>,
	Extension(user): Extension<User>,
	Valid(Json(body)): Valid<Json<SubmitEncryptBody>>,
) -> Result<(StatusCode, String), Error> {
	let Some(incomplete) = DssSign::find_user_incomplete(&state, user.id).await? else {
		let error = Error::default()
			.with_code(StatusCode::BAD_REQUEST)
			.with_message("No active dss sign session found.");

		return Err(error);
	};

	let duration = incomplete.try_into_completed(&state, body.pk, body.r, body.s).await?;
	let message = format!("Correct! This attempt took {duration:?}.");

	Ok((StatusCode::OK, message))
}

async fn get_leaderboard(
	State(state): State<AppState>,
) -> Result<Json<Vec<LeaderboardResult>>, Error> {
	let mut leaderboard = Leaderboard::default();

	for completed in DssSign::find_all_completed(&state).await? {
		let duration = completed.completed_duration()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		if !leaderboard.is_faster_result(completed.user_id, duration) {
			continue;
		}

		let user = User::find_by_id(&state, completed.user_id).await?
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let result = LeaderboardResult {
			username: user.username,
			duration,
		};

		leaderboard.insert(completed.user_id, result);
	}

	Ok(Json(leaderboard.into_results()))
}

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.route("/", get(create_sign))
		.route("/", post(submit_sign))
		.route("/leaderboard", get(get_leaderboard))
}
