mod error;
mod state;
mod middleware;
mod routes;
mod schema;
mod models;
mod leaderboard;
mod math;

use std::env;
use tokio::net::TcpListener;
use axum::Router;

use tower_http::{
	cors::CorsLayer,
	compression::CompressionLayer,
};

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

use crate::state::AppState;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenvy::dotenv()?;

	let port = env::var("PORT")?;
	let addr = format!("[::1]:{port}");

	let state = AppState::init().await?;

	let app = Router::new()
		.merge(routes::guarded_router())
		.layer(axum::middleware::from_fn_with_state(state.clone(), middleware::rate::rate))
		.layer(axum::middleware::from_fn_with_state(state.clone(), middleware::auth::auth))
		.merge(routes::unguarded_router())
		.layer(CompressionLayer::new())
		.layer(CorsLayer::permissive())
		.with_state(state);

	let listener = TcpListener::bind(addr).await?;
	println!("Listening on port {port}...");

	axum::serve(listener, app).await?;

	Ok(())
}
