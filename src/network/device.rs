
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


// FROM: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
#[derive(Debug)]
pub struct DeviceJoinError;
#[derive(Debug)]
pub struct MismatchedMACError;


impl std::fmt::Display for DeviceJoinError {
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(format, "Failed to join DBDevice and Device.")
	}
}


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


pub trait ToDeviceVector
{
	type Item;

	fn to<U>(self, ) -> Vec<U>
	where U: From<Self::Item>;
}


impl<T> ToDeviceVector for Vec<T> {
	type Item = T;

	fn to<U>(self) -> Vec<U>
	where U: From<T>,
	{
		self.into_iter().map(U::from).collect()
	}
}


impl Device
{
	pub fn eq(&self, right: &DBDevice) -> bool
	{
		return self.mac == right.mac;
	}


	pub fn join(&mut self, right: DBDevice) -> ()
	{
		if(self.mac != right.mac)
		{
			return panic!("MAC {} != MAC {}", self.mac, right.mac);
		}

		self.label = right.label;
		self.id = Some(right.id);
		self.band = right.band;
		self.groups = Some(right.groups);
		self.network_id = right.network_id;
		self.static_ip_address = right.static_ip_address;
	}
}
