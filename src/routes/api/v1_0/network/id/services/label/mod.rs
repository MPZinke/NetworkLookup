
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.09.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/



use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::service::Service;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, service::SELECT_Service_by_Network_id_AND_Service_label};


// `/api/v1.0/network/label/{network_id}/services/label`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/label/{network_id}/services/label/{service_label}": "List all services based on service label and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_id}/services/label/{service_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(i32, String)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (network_id, service_label) = path.into_inner();
	let query_response: Result<Vec<Service>, LookupError> = SELECT_Service_by_Network_id_AND_Service_label(pool.as_ref(),
	  network_id, &service_label).await;
	return query_to_response(query_response);
}
