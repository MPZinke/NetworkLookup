
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
use crate::network::{lookup, Device, ToDeviceVector};
use crate::response::ToJsonResponse;
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

	let mut network_devices: Vec<Device> = match(lookup(&pool, id).await)
	{
		Some(network_devices) => network_devices,
		None => return db_devices.to_device_vec().to_json_response(),
	};

	let mut devices: Vec<Device> = vec![];
	for db_device in db_devices
	{
		match(network_devices.iter().position(|network_device|{network_device == &db_device}))
		{
			None => devices.push(db_device.into()),
			Some(network_device_index)
			=>
			{
				let mut device = network_devices.remove(network_device_index);
				device.join(db_device);
				devices.push(device);
			},
		}
	}

	devices.append(&mut network_devices);

	return devices.to_json_response();
}


// `/api/networks/{network_id}/devices/{device_id}`
pub async fn id(path: Path<(i32, i32)>, pool: Data<PgPool>) -> HttpResponse
{
	let (network_id, device_id) = path.into_inner();
	let db_device: DBDevice = match(
		get_device_by_network_id_and_device_id(pool.as_ref(), network_id, device_id).await
	)
	{
		Ok(db_device) => db_device,
		Err(error) => return error.to_json_response(),
	};

	let mut network_devices: Vec<Device> = match(lookup(&pool, network_id).await)
	{
		Some(network_devices) => network_devices,
		None => return Device::from(db_device).to_json_response(),
	};

	let mut device: Device = match(network_devices.iter().position(|network_device|{network_device == &db_device}))
	{
		Some(network_device_index) => network_devices.remove(network_device_index),
		None => return Into::<Device>::into(db_device).to_json_response(),
	};
	return device.join(db_device).to_json_response();
}
