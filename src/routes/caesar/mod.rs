mod encrypt;
mod decrypt;
mod attack;

use axum::Router;
use crate::state::AppState;

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.nest("/encrypt", encrypt::guarded_router())
		.nest("/decrypt", decrypt::guarded_router())
		.nest("/attack", attack::guarded_router())
}
