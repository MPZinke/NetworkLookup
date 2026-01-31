
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
	pub network_id: i32,
	pub ip_address: Option<String>,
	// DB
	pub id: Option<i32>,
	pub band: Option<String>,
	pub groups: Option<Vec<Group>>,
	pub static_ip_address: Option<String>,
}


pub trait ToDeviceVector
{
	fn to_device_vec(self) -> Vec<Device>;
}


impl<T> ToDeviceVector for Vec<T>
where T: From<T>, Device: std::convert::From<T>
{
	fn to_device_vec(self) -> Vec<Device>
	{
		self.into_iter().map(Device::from).collect()
	}
}


impl PartialEq<DBDevice> for Device
{
	fn eq(&self, right: &DBDevice) -> bool
	{
		return self.mac == right.mac;
	}
}


impl Device
{
	pub fn join(&mut self, right: DBDevice) -> &Self
	{
		if(self.mac != right.mac)
		{
			panic!("MAC {} != MAC {}", self.mac, right.mac);
		}

		self.label = right.label;
		self.id = Some(right.id);
		self.band = right.band;
		self.groups = Some(right.groups);
		self.network_id = right.network_id;
		self.static_ip_address = right.static_ip_address;

		return self;
	}
}
