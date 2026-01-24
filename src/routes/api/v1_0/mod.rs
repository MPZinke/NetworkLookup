
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


pub mod group;
pub mod network;


use actix_web::{HttpResponse, http::header::ContentType};


// `/api/v1.0`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/group": "Queries for groups",
		"/api/v1.0/network": "Queries for networks"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);

}