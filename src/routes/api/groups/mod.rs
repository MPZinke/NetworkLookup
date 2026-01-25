
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


use actix_web::{HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::db_tables::Group;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, groups::{get_groups, get_group_by_id}};


// `/api/groups`
pub async fn index(pool: web::Data<PgPool>) -> HttpResponse
{
	let query_response: Result<Vec<Group>, LookupError> = get_groups(pool.as_ref()).await;
	return query_to_response(query_response);
}


// `/api/groups/{id}`
pub async fn id(path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	let id = path.into_inner();
	let query_response: Result<Group, LookupError> = get_group_by_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
