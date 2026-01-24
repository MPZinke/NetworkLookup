
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.08                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod group;


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::Device;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, device::SELECT_Devices_by_Network_label};


// `/api/v1.0/networks/label/{label}/devices`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{	
		"/api/v1.0/network/label/{network_label}/devices/all": "List all devices based on network label",
		"/api/v1.0/network/label/{network_label}/devices/group": "Queries for devices based on group and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/networks/label/{label}/devices/all`
pub async fn all(auth: BearerAuth, path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let label = path.into_inner();
	let query_response: Result<Vec<Device>, LookupError> = SELECT_Devices_by_Network_label(pool.as_ref(), &label).await;
	return query_to_response(query_response);
}
