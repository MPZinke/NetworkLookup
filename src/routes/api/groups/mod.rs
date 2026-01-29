
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


use crate::db_tables::{DBDevice, Group};
use crate::response::ResponseError;
use crate::query::{query_to_response, devices::get_devices_by_group_label, groups::get_groups};


// `/api/groups`
pub async fn index(pool: web::Data<PgPool>) -> HttpResponse
{
	let query_response: Result<Vec<Group>, ResponseError> = get_groups(pool.as_ref()).await;
	return query_to_response(query_response);
}


// `/api/groups/{label}`
pub async fn label(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse
{
	let label = path.into_inner();
	let query_response: Result<Vec<DBDevice>, ResponseError> = get_devices_by_group_label(pool.as_ref(), label).await;
	return query_to_response(query_response);
}
