
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


use crate::db_tables::Service;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, services::{get_services, get_service_by_id}};


// `/api/services`
pub async fn index(pool: web::Data<PgPool>) -> HttpResponse
{
	let query_response: Result<Vec<Service>, LookupError> = get_services(pool.as_ref()).await;
	return query_to_response(query_response);
}


// `/api/services/{id}`
pub async fn id(path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	let id: i32 = path.into_inner();
	let query_response: Result<Service, LookupError> = get_service_by_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
