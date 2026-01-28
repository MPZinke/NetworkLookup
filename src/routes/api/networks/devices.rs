
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


use actix_web::{HttpResponse, web::{Data, Path}};
use sqlx::postgres::PgPool;


use crate::db_tables::DBDevice;
use crate::lookup_error::LookupError;
use crate::network::{lookup, Device};
use crate::response::to_json_response;
use crate::query::query_to_response;
use crate::query::devices::{get_devices_by_network_id, get_device_by_network_id_and_device_id};


// `/api/networks/{id}/devices`
pub async fn index(path: Path<i32>, pool: Data<PgPool>) -> HttpResponse
{
	let id: i32 = path.into_inner();
	let db_devices: Vec<DBDevice> = match(get_devices_by_network_id(pool.as_ref(), id).await)
	{
		Ok(db_devices) => db_devices,
		Err(error) => return error.to_json_response(),
	};

	let devices = match(lookup(&pool, id).await)
	{
		Some(devices) => devices,
		None => return to_json_response(db_devices.into_iter().map(|db_device: DBDevice| db_device.to_device()).collect::<Vec<Device>>()),
	};

	return to_json_response(db_devices);
}


// `/api/networks/{network_id}/devices/{device_id}`
pub async fn id(path: Path<(i32, i32)>, pool: Data<PgPool>) -> HttpResponse
{
	let (network_id, device_id) = path.into_inner();
	let query_response: Result<DBDevice, LookupError> = get_device_by_network_id_and_device_id(
		pool.as_ref(),
		network_id,
		device_id
	).await;
	return query_to_response(query_response);
}
