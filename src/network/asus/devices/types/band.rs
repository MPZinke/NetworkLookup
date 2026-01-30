
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


pub type MacAddresses = Vec<String>;
pub type Interface<T> = HashMap<String, T>;
pub struct InterfacesMacAddresses
{
	pub _2ghz_mac_addresses: MacAddresses,
	pub _5ghz_mac_addresses: MacAddresses,
	pub _ethernet_mac_addresses: MacAddresses,
}


impl InterfacesMacAddresses
{
	pub fn new() -> InterfacesMacAddresses
	{
		return InterfacesMacAddresses {
			_2ghz_mac_addresses: vec![],
			_5ghz_mac_addresses: vec![],
			_ethernet_mac_addresses: vec![],
		}
	}
}


#[derive(Clone, Deserialize, Serialize)]
pub struct WirelessMacAddresses
{
	#[serde(rename = "2G")]
	pub _2ghz_mac_addresses: MacAddresses,
	#[serde(rename = "5G")]
	pub _5ghz_mac_addresses: MacAddresses,
}


impl Default for WirelessMacAddresses
{
	fn default() -> Self
	{
		return WirelessMacAddresses {
			_2ghz_mac_addresses: vec![],
			_5ghz_mac_addresses: vec![],
		}
	}
}
