
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


pub fn to_json_response<T: Serialize>(value: T) -> HttpResponse
{
	return match(serde_json::to_string(&value))
	{
		Ok(json) => HttpResponse::Ok().insert_header(ContentType::json()).body(json),
		Err(error) =>
		{
			let error_message: String = format!(r#"{{"error": "{}"}}"#, error);
			return HttpResponse::InternalServerError().insert_header(ContentType::json()).body(error_message);
		}
	}
}
