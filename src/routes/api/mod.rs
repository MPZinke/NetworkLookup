
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


pub mod v1_0;


use actix_web::{HttpResponse, http::header::ContentType};


// `/api`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0": "The current version of this API"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
