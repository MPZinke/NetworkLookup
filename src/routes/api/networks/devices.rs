
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


use actix_web::{HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::db_tables::Device;
use crate::lookup_error::LookupError;
use crate::query::{query_to_response, devices::{get_devices_by_network_id, get_device_by_network_id_and_device_id}};


// `/api/networks/{id}/devices`
pub async fn index(path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	let id = path.into_inner();
	let query_response: Result<Vec<Device>, LookupError> = get_devices_by_network_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}


// `/api/networks/{network_id}/devices/{device_id}`
pub async fn id(path: web::Path<(i32, i32)>, pool: web::Data<PgPool>) -> HttpResponse
{
	let (network_id, device_id) = path.into_inner();
	let query_response: Result<Device, LookupError> = get_device_by_network_id_and_device_id(
		pool.as_ref(),
		network_id,
		device_id
	).await;
	return query_to_response(query_response);
}
