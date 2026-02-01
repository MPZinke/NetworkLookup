
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2026.01.24                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use actix_web::web;
use sqlx::postgres::PgPool;


mod asus;
mod device;


use asus::{get_asus_network_devices, update_asus_static_devices};
use crate::db_tables::{DBDevice, Network};
use crate::query::networks::get_network_by_id;


pub use device::Device as Device;
pub use device::ToDeviceVector as ToDeviceVector;


pub async fn get_network_devices(pool: &web::Data<PgPool>, network_id: i32) -> Option<Vec<Device>>
{
	let network: Network = get_network_by_id(pool, network_id).await.ok()?;
	let network_type: &String = match(&network.r#type)
	{
		Some(network_type) => network_type,
		None => return None,
	};

	match(network_type.as_str())
	{
		"Asus" => return Some(get_asus_network_devices(&network).await?),
		&_ => return None,
	}
}


pub async fn update_static_devices(pool: &web::Data<PgPool>, network_id: i32, db_devices: &Vec<DBDevice>) -> bool
{
	let network: Network = match(get_network_by_id(pool, network_id).await)
	{
		Ok(network) => network,
		Err(_) => return false,
	};
	let network_type: &String = match(&network.r#type)
	{
		Some(network_type) => network_type,
		None => return false,
	};

	match(network_type.as_str())
	{
		"Asus" => return update_asus_static_devices(db_devices, &network).await,
		&_ => return false,
	}
}
