
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


use reqwest::{Client, Response};
use serde::Deserialize;


use crate::network::Device;


use crate::network::asus::devices::types::band::{InterfacesMacAddresses, Interface, MacAddresses, WirelessMacAddresses};


async fn get_devices_bands_raw_data(asus_token: &String, network_gateway: &String) -> Option<String>
{
	let client = Client::new();
	let response: Response = client
	.get(format!("http://{}/ajax_onboarding.asp", network_gateway))
	.header("Cookie", asus_token)
	.header("Referer", format!("http://{}/device-map/clients.asp", network_gateway))
	.send()
	.await
	.ok()?;

	return response.text().await.ok();
}


fn parse_devices_bands_raw_data(raw_data: String) -> InterfacesMacAddresses
{
	let mut wireless_mac_addresses_option: Option<WirelessMacAddresses> = None;
	let mut wired_mac_addresses_option: Option<MacAddresses> = None;

	let lines: std::str::Split<char> = raw_data.split('\n');
	for line in lines
	{
		if(line.starts_with("get_wclientlist = ["))
		{
			wireless_mac_addresses_option = strip_and_convert_to_json(line, "get_wclientlist = [", "][0];");
		}
		else if(line.starts_with("get_wiredclientlist = ["))
		{
			wired_mac_addresses_option = strip_and_convert_to_json(line, "get_wiredclientlist = [", "][0];");
		}
	}

	let wireless_mac_addresses: WirelessMacAddresses = wireless_mac_addresses_option.unwrap_or_default();
	let ethernet_mac_addresses: MacAddresses = wired_mac_addresses_option.unwrap_or(vec![]);

	return InterfacesMacAddresses {
		_2ghz_mac_addresses: wireless_mac_addresses._2ghz_mac_addresses,
		_5ghz_mac_addresses: wireless_mac_addresses._5ghz_mac_addresses,
		_ethernet_mac_addresses: ethernet_mac_addresses,
	}
}


fn add_bands_for_interfaces(devices: &mut Vec<Device>, interfaces_mac_addresses: InterfacesMacAddresses) -> ()
{
	let some_all = Some("All".to_string());
	for device in &mut *devices
	{
		if(device.band == some_all)
		{
			continue;
		}
		else if(interfaces_mac_addresses._2ghz_mac_addresses.contains(&device.mac))
		{
			device.band = Some(String::from("2.4GHz"));
		}
		else if(interfaces_mac_addresses._5ghz_mac_addresses.contains(&device.mac))
		{
			device.band = Some(String::from("5GHz"));
		}
		else if(interfaces_mac_addresses._ethernet_mac_addresses.contains(&device.mac))
		{
			device.band = Some(String::from("Ethernet"));
		}
	}
}


fn strip_and_convert_to_json<T>(string: &str, prefix: &str, suffix: &str) -> Option<T>
where T: for<'a> Deserialize<'a>
{
	let json = string.strip_prefix(prefix)?.strip_suffix(suffix)?;
	match(serde_json::from_str::<Interface<T>>(&json))
	{
		Ok(object) => return object.into_values().next(),
		Err(error)
		=>
		{
			println!("{}", error);
			return None;
		},
	}
}


pub async fn add_devices_bands(asus_token: &String, network_gateway: &String, devices: &mut Vec<Device>) -> ()
{
	let raw_data: String = match(get_devices_bands_raw_data(asus_token, network_gateway).await)
	{
		Some(raw_data) => raw_data,
		None => return,
	};
	let interfaces_mac_addresses: InterfacesMacAddresses = parse_devices_bands_raw_data(raw_data);
	add_bands_for_interfaces(devices, interfaces_mac_addresses);
}
