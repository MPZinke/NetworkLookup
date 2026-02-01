
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


pub mod groups;
pub mod networks;
pub mod services;


use actix_web::{HttpResponse, http::header::ContentType};


// `GET /api`
pub async fn get_index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/groups": "Queries for groups",
		"/api/networks": "Queries for networks",
		"/api/services": "Queries for services"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}