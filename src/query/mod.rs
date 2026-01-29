
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.09                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod devices;
pub mod groups;
pub mod networks;
pub mod services;


use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};
use serde_json;
use serde::Serialize;


use crate::response::ResponseError;


/*
SUMMARY: Unwraps the Result and returns an HttpResponse type.
PARAMS:  Takes the generic Result to unwrap.
DETAILS: Unwraps the Result. If Result is Ok, then it attempts to convert the result to a JSON. If the Result is an
         error or fails to convert the result to a JSON, the corresponding response type is selected and the error
         is sent as a JSON.
RETURNS: An HttpResponse with a value or error body.
*/
pub fn query_to_response<T: Serialize>(generic_query: Result<T, ResponseError>) -> HttpResponse
{
	let response_generic: T = match(generic_query)
	{
		Ok(response_generic) => response_generic,
		Err(error) =>
		{
			let response: fn() -> HttpResponseBuilder = match(error)
			{
				ResponseError::NotFound(_) => HttpResponse::NotFound,
				ResponseError::InvalidHeader(_) => HttpResponse::InternalServerError,
				ResponseError::Postgres(_) => HttpResponse::InternalServerError,
				ResponseError::Request(_) => HttpResponse::InternalServerError
			};
			let error_message: String = format!(r#"{{"error": "{}"}}"#, error);
			return response().insert_header(ContentType::json()).body(error_message);
		}
	};

	match(serde_json::to_string(&response_generic))
	{
		Ok(response_body) => return HttpResponse::Ok().insert_header(ContentType::json()).body(response_body),
		Err(error) => 
		{
			let error_message: String = format!(r#"{{"error": "{}"}}"#, error);
			return HttpResponse::InternalServerError().insert_header(ContentType::json()).body(error_message);
		}
	}
}
