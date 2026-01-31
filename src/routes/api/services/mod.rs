
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


use actix_web::{HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::response::ToJsonResponse;
use crate::query::services::{get_services, get_service_by_id};


// `/api/services`
pub async fn index(pool: web::Data<PgPool>) -> HttpResponse
{
	return get_services(pool.as_ref()).await.to_json_response();
}


// `/api/services/{id}`
pub async fn id(path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	let id: i32 = path.into_inner();
	return get_service_by_id(pool.as_ref(), id).await.to_json_response();
}
