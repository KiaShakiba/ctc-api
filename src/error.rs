use std::{
	fmt::{self, Display},
	env::VarError,
	num::ParseIntError,
};

use diesel::result::Error as DieselError;
use deadpool_diesel::InteractError;
use paper_client::PaperClientError;
use postcard::Error as PostcardError;

use axum::{
	Error as AxumError,
	extract::multipart::MultipartError,
	response::{Response, IntoResponse},
	http::{
		StatusCode,
		HeaderValue,
		header::CONTENT_TYPE,
	},
};

#[derive(Debug)]
pub struct Error {
	code: StatusCode,
	message: String,
}

impl Error {
	pub fn set_code(&mut self, code: StatusCode) {
		self.code = code;
	}

	pub fn with_code(mut self, code: StatusCode) -> Self {
		self.set_code(code);
		self
	}

	pub fn set_message(&mut self, message: impl Display) {
		self.message = message.to_string();
	}

	pub fn with_message(mut self, message: impl Display) -> Self {
		self.set_message(message);
		self
	}
}

impl std::error::Error for Error {}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.code.as_u16(), self.message)
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let mut response = Response::new(self.message.into());
		*response.status_mut() = self.code;

		response
			.headers_mut()
			.insert(CONTENT_TYPE, HeaderValue::from_static("plain/text"));

		response
	}
}

impl PartialEq<StatusCode> for Error {
	fn eq(&self, code: &StatusCode) -> bool {
		self.code.eq(code)
	}
}

impl PartialEq<StatusCode> for &Error {
	fn eq(&self, code: &StatusCode) -> bool {
		self.code.eq(code)
	}
}

impl Default for Error {
	fn default() -> Self {
		StatusCode::INTERNAL_SERVER_ERROR.into()
	}
}

impl From<VarError> for Error {
	fn from(_: VarError) -> Self {
		Error::default()
	}
}

impl From<ParseIntError> for Error {
	fn from(_: ParseIntError) -> Self {
		Error::default()
	}
}

impl From<DieselError> for Error {
	fn from(_: DieselError) -> Self {
		Error::default()
	}
}

impl From<InteractError> for Error {
	fn from(_: InteractError) -> Self {
		Error::default()
	}
}

impl From<PaperClientError> for Error {
	fn from(_: PaperClientError) -> Self {
		Error::default()
	}
}

impl From<PostcardError> for Error {
	fn from(_: PostcardError) -> Self {
		Error::default()
	}
}

impl From<AxumError> for Error {
	fn from(_: AxumError) -> Self {
		Error::default()
	}
}

impl From<MultipartError> for Error {
	fn from(_: MultipartError) -> Self {
		Error::default()
	}
}

impl From<StatusCode> for Error {
	fn from(code: StatusCode) -> Self {
		Error {
			code,
			message: get_default_code_message(code).into(),
		}
	}
}

fn get_default_code_message(code: StatusCode) -> &'static str {
	match code {
		StatusCode::NOT_FOUND => "Not found",
		StatusCode::UNAUTHORIZED => "Unauthorized",
		StatusCode::FORBIDDEN => "Forbidden",
		_ => "An error occurred. Please try again later.",
	}
}
