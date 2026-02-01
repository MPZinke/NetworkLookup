
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2026.01.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use std::collections::HashMap;


use serde::{Deserialize, Serialize};


use crate::network::Device;


#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Serialize)]
pub struct AsusDevice
{
	pub name: String,
	pub nickName: String,
	pub ip: String,
	pub mac: String,
	pub isOnline: String,
}


impl From<AsusDevice> for Device
{
	fn from(asus_device: AsusDevice) -> Device
	{
		let label: String;
		if(asus_device.nickName.len() > 0)
		{
			label = asus_device.nickName;
		}
		else
		{
			label = asus_device.name;
		}
		return Device {
			// Network and DB
			label: label,
			mac: asus_device.mac,
			// Network
			ip_address: Some(asus_device.ip),
			// DB
			id: None,
			band: None,
			network_id: -1,
			static_ip_address: None,
			groups: None,
		};
	}
}


#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]  // FROM: https://github.com/serde-rs/serde/issues/1386#issuecomment-759540656
pub enum NetworkMapValue
{
	AsusDevice(AsusDevice),
	MacList(Vec<String>),
	ClientAPILevel(String),
}


pub type NetworkMap = HashMap<String, NetworkMapValue>;
