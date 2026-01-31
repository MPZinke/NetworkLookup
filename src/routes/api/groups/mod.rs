
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


use crate::query::{devices::get_devices_by_group_label, groups::get_groups};
use crate::response::ToJsonResponse;


// `/api/groups`
pub async fn index(pool: web::Data<PgPool>) -> HttpResponse
{
	return get_groups(pool.as_ref()).await.to_json_response();
}


// `/api/groups/{label}`
pub async fn label(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse
{
	let label = path.into_inner();
	return get_devices_by_group_label(pool.as_ref(), label).await.to_json_response();
}
