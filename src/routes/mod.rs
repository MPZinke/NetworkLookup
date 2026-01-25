
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.05                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod api;


use actix_web::{HttpResponse, http::header::ContentType};


// `/`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api": "The API for all active protocols"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
