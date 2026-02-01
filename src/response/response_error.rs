
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


use serde::ser::{Serialize, Serializer, SerializeMap};


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


// FROM: https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
impl Serialize for ResponseError
{
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where S: Serializer,
	{
		let mut map = serializer.serialize_map(Some(1))?;
		map.serialize_entry("error", &format!("{}", self))?;
		return map.end();
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
