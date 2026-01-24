
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


pub mod id;
pub mod label;


use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::group::Group;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, group::SELECT_Groups};


// `/api/v1.0/group`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/group/all": "List all groups",
		"/api/v1.0/group/id": "Queries for group based on id",
		"/api/v1.0/group/label": "Queries for group based on label"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/group/all`
pub async fn all(auth: BearerAuth, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let query_response: Result<Vec<Group>, LookupError> = SELECT_Groups(pool.as_ref()).await;
	return query_to_response(query_response);
}
