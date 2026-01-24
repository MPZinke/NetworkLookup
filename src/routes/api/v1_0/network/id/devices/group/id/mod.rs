
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


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::device::Device;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, device::SELECT_Devices_by_Network_id_AND_Group_id};


// `/api/v1.0/network/id/{network_id}/devices/group/id`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/id/{network_id}/devices/group/id/{group_id}": "List all devices based on group id and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}/devices/group/id/{group_id}`
pub async fn id(auth: BearerAuth, path: web::Path<(i32, i32)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_id, Group_id) = path.into_inner();
	let query_response: Result<Vec<Device>, LookupError> = SELECT_Devices_by_Network_id_AND_Group_id(pool.as_ref(),
	  Network_id, Group_id).await;
	return query_to_response(query_response);
}
