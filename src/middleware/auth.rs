use axum::{
	extract::State,
	http::{header, StatusCode, Request},
	middleware::Next,
	body::Body,
	response::Response,
};

use crate::{
	error::Error,
	state::AppState,
	models::user::User,
};

pub async fn auth(
	State(state): State<AppState>,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response<Body>, Error> {
	let bearer_token = req.headers()
		.get(header::AUTHORIZATION)
		.and_then(|auth_header| auth_header.to_str().ok())
		.and_then(|auth_value| {
			if !auth_value.starts_with("Bearer ") {
				return None;
			}

			Some(auth_value[7..].to_owned())
		})
		.ok_or(StatusCode::UNAUTHORIZED)?;

	let user = User::find_by_bearer(&state, &bearer_token).await?
		.ok_or(StatusCode::UNAUTHORIZED)?;

	req.extensions_mut().insert(user);
	Ok(next.run(req).await)
}
