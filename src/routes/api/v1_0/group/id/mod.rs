
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


use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::Group;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, group::SELECT_Group_by_id};


// `/api/v1.0/group/id`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/group/id/{group_id}": "Get a group by ID"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/group/id/{group_id}`
pub async fn id(auth: BearerAuth, path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let id = path.into_inner();
	let query_response: Result<Group, LookupError> = SELECT_Group_by_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
