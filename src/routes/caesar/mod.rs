mod encrypt;
mod decrypt;

use axum::Router;
use crate::state::AppState;

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.nest("/encrypt", encrypt::guarded_router())
		.nest("/decrypt", decrypt::guarded_router())
}
