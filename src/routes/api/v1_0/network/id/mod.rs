
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.07                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod device;
pub mod devices;
pub mod services;


use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::Network;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, network::SELECT_Network_by_id};


// `/api/v1.0/network/id`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/id/{network_id}": "Get a network by id",
		"/api/v1.0/network/id/{network_id}/device": "Queries for device based on network id",
		"/api/v1.0/network/id/{network_id}/devices": "Queries for devices based one network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}`
pub async fn id(auth: BearerAuth, path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let id = path.into_inner();
	let query_response: Result<Network, LookupError> = SELECT_Network_by_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
