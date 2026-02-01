
/***********************************************************************************************************************
**
* created by: MPZinke*
* on 2022.05.05*
**
* DESCRIPTION: TEMPLATE*
* BUGS:*
* FUTURE:*
**
***********************************************************************************************************************/


use serde::Serialize;
use sqlx::{FromRow, Row, postgres::PgRow};


use crate::network::Device as NetworkDevice;


pub type Group = String;


#[derive(Debug, Serialize)]
pub struct Device
{
	// Network and DB
	pub label: String,
	pub mac: String,
	// DB
	pub id: i32,
	pub band: Option<String>,
	pub groups: Vec<Group>,
	pub network_id: i32,
	pub static_ip_address: Option<String>,
}


// FROM: https://stackoverflow.com/a/78618913
impl<'r> FromRow<'r, PgRow> for Device
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error>
    {
		Ok(
			Device {
				// Network and DB
				label: row.try_get::<String, &str>("label")?,
				mac: row.try_get::<String, &str>("mac")?,
				// DB
				id: row.try_get::<i32, &str>("id")?,
				band: row.try_get::<Option<String>, &str>("band")?,
				groups: row.try_get::<String, &str>("groups")?.split(',').map(|s| s.to_string()).collect(),
				network_id: row.try_get::<i32, &str>("Networks.id")?,
				static_ip_address: row.try_get::<Option<String>, &str>("static_ip_address")?,
			}
		)
	}
}


impl From<Device> for NetworkDevice
{
	fn from(db_device: Device) -> NetworkDevice
	{
		return NetworkDevice {
			// Network and DB
			label: db_device.label,
			mac: db_device.mac,
			// Network
			ip_address: None,
			// DB
			id: Some(db_device.id),
			band: db_device.band,
			groups: Some(db_device.groups),
			network_id: db_device.network_id,
			static_ip_address: db_device.static_ip_address,
		};
	}
}


impl Device
{
	pub fn to_static_list_string(&self) -> String
	{
		return match(&self.static_ip_address)
		{
			Some(static_ip_address) => format!("<{}>{}>>{}", self.mac, static_ip_address, self.label),
			None => String::from(""),
		}
	}
}
