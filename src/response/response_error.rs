
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
pub enum ResponseError
{
	InvalidHeader(reqwest::header::InvalidHeaderValue),
	NotFound(std::io::Error),
	Postgres(sqlx::error::Error),
	Request(reqwest::Error),
}


impl ResponseError
{
	pub fn to_json_response(self) -> HttpResponse
	{
		let response: fn() -> HttpResponseBuilder = match(self)
		{
			ResponseError::NotFound(_) => HttpResponse::NotFound,
			ResponseError::InvalidHeader(_) => HttpResponse::InternalServerError,
			ResponseError::Postgres(_) => HttpResponse::InternalServerError,
			ResponseError::Request(_) => HttpResponse::InternalServerError
		};
		let error_message: String = format!(r#"{{"error": "{}"}}"#, self);
		return response().insert_header(ContentType::json()).body(error_message);
	}
}


impl std::fmt::Display for ResponseError
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match(self)
		{
			ResponseError::InvalidHeader(error) => write!(format, "{}", error),
			ResponseError::NotFound(error) => write!(format, "{}", error),
			ResponseError::Postgres(error) => write!(format, "{}", error),
			ResponseError::Request(error) => write!(format, "{}", error),
		}
	}
}


impl From<reqwest::header::InvalidHeaderValue> for ResponseError
{
	fn from(err: reqwest::header::InvalidHeaderValue) -> Self
	{
		ResponseError::InvalidHeader(err)
	}
}


impl From<std::io::Error> for ResponseError
{
	fn from(err: std::io::Error) -> Self
	{
		ResponseError::NotFound(err)
	}
}


impl From<sqlx::error::Error> for ResponseError
{
	fn from(err: sqlx::error::Error) -> Self
	{
		ResponseError::Postgres(err)
	}
}


impl From<reqwest::Error> for ResponseError
{
	fn from(err: reqwest::Error) -> Self
	{
		ResponseError::Request(err)
	}
}
