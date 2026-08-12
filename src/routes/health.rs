use axum::{Router, routing::get};

use crate::{error::Error, state::AppState};

async fn get_health() -> Result<(), Error> {
	Ok(())
}

pub fn guarded_router() -> Router<AppState> {
	Router::new()
}

pub fn unguarded_router() -> Router<AppState> {
	Router::new().route("/", get(get_health))
}
