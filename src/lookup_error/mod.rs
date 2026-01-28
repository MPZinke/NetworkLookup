
/***********************************************************************************************************************
*																													  *
*   created by: MPZinke																								*
*   on 2022.05.24																									  *
*																													  *
*   DESCRIPTION: TEMPLATE																							  *
*   BUGS:																											  *
*   FUTURE:																											*
*																													  *
***********************************************************************************************************************/

use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};


// ——————————————————————————————————————————————————— ERROR ENUM ——————————————————————————————————————————————————— //

// FROM: https://fettblog.eu/rust-enums-wrapping-errors/
//  AND: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html
#[derive(Debug)]
pub enum LookupError
{
	InvalidHeader(reqwest::header::InvalidHeaderValue),
	NotFound(std::io::Error),
	Postgres(sqlx::error::Error),
	Request(reqwest::Error),
}


impl LookupError
{
	pub fn to_json_response(self) -> HttpResponse
	{
		let response: fn() -> HttpResponseBuilder = match(self)
		{
			LookupError::NotFound(_) => HttpResponse::NotFound,
			LookupError::InvalidHeader(_) => HttpResponse::InternalServerError,
			LookupError::Postgres(_) => HttpResponse::InternalServerError,
			LookupError::Request(_) => HttpResponse::InternalServerError
		};
		let error_message: String = format!(r#"{{"error": "{}"}}"#, self);
		return response().insert_header(ContentType::json()).body(error_message);
	}
}


impl std::fmt::Display for LookupError
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match(self)
		{
			LookupError::InvalidHeader(error) => write!(format, "{}", error),
			LookupError::NotFound(error) => write!(format, "{}", error),
			LookupError::Postgres(error) => write!(format, "{}", error),
			LookupError::Request(error) => write!(format, "{}", error),
		}
	}
}


impl From<reqwest::header::InvalidHeaderValue> for LookupError
{
	fn from(err: reqwest::header::InvalidHeaderValue) -> Self
	{
		LookupError::InvalidHeader(err)
	}
}


impl From<std::io::Error> for LookupError
{
	fn from(err: std::io::Error) -> Self
	{
		LookupError::NotFound(err)
	}
}


impl From<sqlx::error::Error> for LookupError
{
	fn from(err: sqlx::error::Error) -> Self
	{
		LookupError::Postgres(err)
	}
}


impl From<reqwest::Error> for LookupError
{
	fn from(err: reqwest::Error) -> Self
	{
		LookupError::Request(err)
	}
}
