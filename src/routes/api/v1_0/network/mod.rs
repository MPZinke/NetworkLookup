
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


pub mod id;
pub mod label;


use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::Network;
use crate::query::{query_to_response, network::SELECT_Networks};
use crate::lookup_error::LookupError;


// `/api/v1.0/network`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/all": "List all networks",
		"/api/v1.0/network/id": "Queries for a network based on network id",
		"/api/v1.0/network/label": "Queries for a network based on network label"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/all`
pub async fn all(auth: BearerAuth, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let query_response: Result<Vec<Network>, LookupError> = SELECT_Networks(pool.as_ref()).await;
	return query_to_response(query_response);
}
