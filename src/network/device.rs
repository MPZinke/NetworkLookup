
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2026.01.26                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use serde::{Deserialize, Serialize};


use crate::db_tables::{DBDevice, Group};


#[derive(Deserialize, Serialize)]
pub struct Device
{
	// Network and DB
	pub label: String,
	pub mac: String,
	// Network
	pub ip_address: Option<String>,
	// DB
	pub id: Option<i32>,
	pub band: Option<String>,
	pub groups: Option<Vec<Group>>,
	pub network_id: i32,
	pub static_ip_address: Option<String>,
}


impl From<DBDevice> for Device
{
	fn from(db_device: DBDevice) -> Device
	{
		return Device {
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


// FROM: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
#[derive(Debug)]
struct DeviceJoinError;
#[derive(Debug)]
struct MismatchedMACError;


impl std::fmt::Display for DeviceJoinError {
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(format, "Failed to join DBDevice and Device.")
	}
}


impl Device
{
	fn eq(self, right: &DBDevice) -> bool
	{
		return self.mac == right.mac;
	}


	fn join(mut self, right: DBDevice) -> Result<(), DeviceJoinError>
	{
		if(self.mac != right.mac)
		{
			return Err(DeviceJoinError);
		}

		self.label = right.label;
		self.id = Some(right.id);
		self.band = right.band;
		self.groups = Some(right.groups);
		self.network_id = right.network_id;
		self.static_ip_address = right.static_ip_address;

		Ok(())
	}
}
