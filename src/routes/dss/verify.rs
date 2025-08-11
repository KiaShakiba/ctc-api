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
		dss::{DssVerify, DssVerifyPublic},
	},
	leaderboard::{Leaderboard, LeaderboardResult},
};

#[derive(Deserialize, Validate)]
struct SubmitVerifyBody {
	u: u64,
	v: u64,
	w: u64,
}

async fn create_verify(
	State(state): State<AppState>,
	Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<DssVerifyPublic>), Error> {
	if let Some(existing) = DssVerify::find_user_incomplete(&state, user.id).await? {
		let public: DssVerifyPublic = existing.into();
		return Ok((StatusCode::OK, Json(public)));
	}

	let verify = DssVerify::create(&state, user.id).await?;
	let public: DssVerifyPublic = verify.into();

	Ok((StatusCode::CREATED, Json(public)))
}

async fn submit_verify(
	State(state): State<AppState>,
	Extension(user): Extension<User>,
	Valid(Json(body)): Valid<Json<SubmitVerifyBody>>,
) -> Result<(StatusCode, String), Error> {
	let Some(incomplete) = DssVerify::find_user_incomplete(&state, user.id).await? else {
		let error = Error::default()
			.with_code(StatusCode::BAD_REQUEST)
			.with_message("No active dss verify session found.");

		return Err(error);
	};

	let duration = incomplete.try_into_completed(&state, body.u, body.v, body.w).await?;
	let message = format!("Correct! This attempt took {duration:?}.");

	Ok((StatusCode::OK, message))
}

async fn get_leaderboard(
	State(state): State<AppState>,
) -> Result<Json<Vec<LeaderboardResult>>, Error> {
	let mut leaderboard = Leaderboard::default();

	for completed in DssVerify::find_all_completed(&state).await? {
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
		.route("/", get(create_verify))
		.route("/", post(submit_verify))
		.route("/leaderboard", get(get_leaderboard))
}
