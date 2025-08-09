mod encrypt;

use axum::Router;
use crate::state::AppState;

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.nest("/encrypt", encrypt::guarded_router())
}
