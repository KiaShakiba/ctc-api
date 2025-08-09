mod user;
mod caesar;
mod diffie_hellman_exchange;
mod rsa;
mod dss;

use axum::Router;
use crate::state::AppState;

pub fn guarded_router() -> Router<AppState> {
	Router::new()
		.nest("/user", user::guarded_router())
		.nest("/caesar", caesar::guarded_router())
		.nest("/diffie-hellman", diffie_hellman_exchange::guarded_router())
		.nest("/rsa", rsa::guarded_router())
		.nest("/dss", dss::guarded_router())
}

pub fn unguarded_router() -> Router<AppState> {
	Router::new()
		.nest("/user", user::unguarded_router())
}
