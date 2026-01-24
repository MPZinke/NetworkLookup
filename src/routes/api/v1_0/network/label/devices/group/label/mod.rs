
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


use crate::db_tables::Device;
use crate::query::{query_to_response, device::SELECT_Devices_by_Network_label_AND_Group_label};
use crate::lookup_error::LookupError;


// `/api/v1.0/network/label/{network_label}/devices/group`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/label/{network_label}/devices/group/label/{group_label}": "List all devices based on group label and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/devices/group/label/{group_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(String, String)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_label, Group_label) = path.into_inner();
	let query_response: Result<Vec<Device>, LookupError> = SELECT_Devices_by_Network_label_AND_Group_label(pool.as_ref(),
	  &Network_label, &Group_label).await;
	return query_to_response(query_response);
}
