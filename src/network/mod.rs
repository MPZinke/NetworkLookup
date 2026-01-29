
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


use asus::get_devices;
use crate::db_tables::Network;
use crate::query::networks::get_network_by_id;


pub use device::Device as Device;
pub use device::ToDeviceVector as ToDeviceVector;


pub async fn lookup(pool: &web::Data<PgPool>, network_id: i32) -> Option<Vec<Device>>
{
	let network: Network = get_network_by_id(pool, network_id).await.ok()?;
	let network_type: &String = match(&network.r#type)
	{
		Some(network_type) => network_type,
		None => return None,
	};

	match(network_type.as_str())
	{
		"Asus" => return Some(get_devices(&network).await?),
		&_ => return None,
	}
}
