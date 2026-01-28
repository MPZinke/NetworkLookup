
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


use crate::network::Device;


pub type Group = String;


#[derive(Debug, Serialize)]
pub struct DBDevice
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
impl<'r> FromRow<'r, PgRow> for DBDevice
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error>
    {
		Ok(
			DBDevice {
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


impl DBDevice
{
	pub fn to_device(self) -> Device
	{
		return Device {
			// Network and DB
			label: self.label,
			mac: self.mac,
			// Network
			ip_address: None,
			// DB
			id: Some(self.id),
			band: self.band,
			groups: Some(self.groups),
			network_id: self.network_id,
			static_ip_address: self.static_ip_address,
		};
	}
}
