mod error;
mod leaderboard;
mod math;
mod middleware;
mod models;
mod routes;
mod schema;
mod state;

use std::env;

use axum::Router;
use mimalloc::MiMalloc;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use crate::state::AppState;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenvy::dotenv()?;

	let subscriber = tracing_subscriber::FmtSubscriber::new();
	tracing::subscriber::set_global_default(subscriber)?;

	let host = env::var("HOST")?;
	let port = env::var("PORT")?;
	let addr = format!("{host}:{port}");

	let state = AppState::init().await?;

	let app = Router::new()
		.merge(routes::guarded_router())
		.layer(axum::middleware::from_fn_with_state(
			state.clone(),
			middleware::rate::rate,
		))
		.layer(axum::middleware::from_fn_with_state(
			state.clone(),
			middleware::auth::auth,
		))
		.merge(routes::unguarded_router())
		.layer(CompressionLayer::new())
		.layer(CorsLayer::permissive())
		.with_state(state);

	let listener = TcpListener::bind(addr).await?;
	tracing::info!("listening on port {port}...");

	axum::serve(listener, app).await?;

	Ok(())
}
