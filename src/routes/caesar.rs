use axum::{
	Router,
	extract::{State, Json, Extension},
	routing::{get, post},
	http::StatusCode,
	response::IntoResponse,
};

use crate::{
	error::Error,
	state::AppState,
	models::user::User,
};

pub async fn create_encypt(
	Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
}

pub async fn submit_encypt(
	Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
}

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.route("/encrypt", get(create_encypt))
		.route("/encrypt", post(submit_encypt))
}
