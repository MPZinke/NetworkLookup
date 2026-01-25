
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


pub mod devices;


use actix_web::{HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::db_tables::Network;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, networks::{get_networks, get_network_by_id}};


// `/api/networks`
pub async fn index(pool: web::Data<PgPool>) -> HttpResponse
{
	let query_response: Result<Vec<Network>, LookupError> = get_networks(pool.as_ref()).await;
	return query_to_response(query_response);
}


// `/api/networks/{network_id}`
pub async fn id(path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	let id: i32 = path.into_inner();
	let query_response: Result<Network, LookupError> = get_network_by_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
