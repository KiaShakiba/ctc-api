mod sign;
mod verify;

use axum::Router;
use crate::state::AppState;

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.nest("/sign", sign::guarded_router())
		.nest("/verify", verify::guarded_router())
}
