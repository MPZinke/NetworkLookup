
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2026.01.27                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use actix_web::{http::header::ContentType, HttpResponse};
use serde_json;
use serde::Serialize;


mod response_error;


pub use response_error::ResponseError as ResponseError;


pub trait ToJsonResponse
{
	fn to_json_response(self) -> HttpResponse;
}


impl<T> ToJsonResponse for std::vec::Vec<T>
where T: Serialize,
{
	fn to_json_response(self) -> HttpResponse
	{
		return match(serde_json::to_string(&self))
		{
			Ok(json) => HttpResponse::Ok().insert_header(ContentType::json()).body(json),
			Err(error) =>
			{
				let error_message: String = format!(r#"{{"error": "{}"}}"#, error);
				return HttpResponse::InternalServerError().insert_header(ContentType::json()).body(error_message);
			}
		}
	}
}
