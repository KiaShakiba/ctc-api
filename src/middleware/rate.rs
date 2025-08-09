use std::env;
use serde::{Serialize, Deserialize};
use kwik::time;

use axum::{
	extract::{State, Extension},
	http::{StatusCode, Request},
	middleware::Next,
	body::Body,
	response::Response,
};

use crate::{
	error::Error,
	state::{AppState, Cachable},
	models::user::User,
};

#[derive(Debug, Serialize, Deserialize)]
struct AccessPeriod {
	start: u64,
	count: u32,
}

pub async fn rate(
	State(state): State<AppState>,
	Extension(user): Extension<User>,
	req: Request<Body>,
	next: Next,
) -> Result<Response<Body>, Error> {
	let now = time::timestamp();

	let error = Error::default()
		.with_code(StatusCode::TOO_MANY_REQUESTS)
		.with_message("You are making too many requests. Slow down!");

	let mut period = AccessPeriod::from_cached(state.cache(), user.id)?
		.unwrap_or_else(|| AccessPeriod::new(now));

	let max_count = env::var("MAX_REQUESTS_PER_SECOND")?.parse::<u32>()?;
	let is_exceeded = period.is_exceeded(now, max_count);

	period.to_cached(state.cache(), user.id)?;

	if is_exceeded {
		return Err(error);
	}

	Ok(next.run(req).await)
}

impl AccessPeriod {
	fn new(now: u64) -> Self {
		AccessPeriod {
			start: now,
			count: 1,
		}
	}

	fn is_exceeded(&mut self, now: u64, max_count: u32) -> bool {
		self.count += 1;

		if now - self.start > 1_000 {
			self.start = now;
			self.count = 0;

			return false;
		}

		self.count > max_count
	}
}

impl Cachable for AccessPeriod {
	type Id = i32;

	fn cache_key(id: Self::Id) -> String {
		format!("user:{id}:rate")
	}
}
