
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


use actix_web::{HttpResponse, http::header::ContentType, web::{Data, Path}};
use sqlx::postgres::PgPool;


use crate::db_tables::DBDevice;
use crate::network::{get_network_devices, update_allowed_devices, update_static_devices, Device, ToDeviceVector};
use crate::query::devices::{get_devices_by_network_id, get_device_by_network_id_and_device_id};
use crate::response::ToJsonResponse;


// `GET /api/networks/{id}/devices`
pub async fn get_index(path: Path<i32>, pool: Data<PgPool>) -> HttpResponse
{
	let id: i32 = path.into_inner();
	let db_devices: Vec<DBDevice> = match(get_devices_by_network_id(pool.as_ref(), id).await)
	{
		Ok(db_devices) => db_devices,
		Err(error) => return error.to_json_response(),
	};

	let mut network_devices: Vec<Device> = match(get_network_devices(&pool, id).await)
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


// `GET /api/networks/{id}/devices/allowed`
pub async fn get_allowed() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/networks/{id}/devices/allowed/2.4GHz": "Update the allowed devices for the 2.4GHz band",
		"/api/networks/{id}/devices/allowed/5GHz": "Update the allowed devices for the 5GHz band"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `POST /api/networks/{id}/devices/allowed/[(2.4|5)GHz]`
pub fn post_allowed(band: &'static str) -> impl Fn(Path<i32>, Data<PgPool>) -> std::pin::Pin<Box<dyn std::future::Future<Output = HttpResponse> + Send>>
	+ Clone
	+ 'static
{
	if(band != "2.4GHz" && band != "5GHz")
	{
		panic!("Bad band: '{}'", band);
	}

	move |path, pool|
	{
        Box::pin(
        	async move
        	{
				let id: i32 = path.into_inner();
				let db_devices: Vec<DBDevice> = match(get_devices_by_network_id(pool.as_ref(), id).await)
				{
					Ok(db_devices) => db_devices,
					Err(error) => return error.to_json_response(),
				};

				return update_allowed_devices(&pool, id, band, &db_devices).await.to_json_response();
        	}
        )
    }
}



// `POST /api/networks/{id}/devices/static`
pub async fn post_static(path: Path<i32>, pool: Data<PgPool>) -> HttpResponse
{
	let id: i32 = path.into_inner();
	let db_devices: Vec<DBDevice> = match(get_devices_by_network_id(pool.as_ref(), id).await)
	{
		Ok(db_devices) => db_devices,
		Err(error) => return error.to_json_response(),
	};

	return update_static_devices(&pool, id, &db_devices).await.to_json_response();
}


// `GET /api/networks/{network_id}/devices/{device_id}`
pub async fn get_id(path: Path<(i32, i32)>, pool: Data<PgPool>) -> HttpResponse
{
	let (network_id, device_id) = path.into_inner();
	let db_device: DBDevice = match(
		get_device_by_network_id_and_device_id(pool.as_ref(), network_id, device_id).await
	)
	{
		Ok(db_device) => db_device,
		Err(error) => return error.to_json_response(),
	};

	let mut network_devices: Vec<Device> = match(get_network_devices(&pool, network_id).await)
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
