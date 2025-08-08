mod user;
mod caesar;

use axum::Router;
use crate::state::AppState;

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.nest("/user", user::guarded_router())
		.nest("/caesar", caesar::guarded_router())
}

pub fn unguarded_router() -> Router<AppState> {
	Router::new()
		.nest("/user", user::unguarded_router())
}
