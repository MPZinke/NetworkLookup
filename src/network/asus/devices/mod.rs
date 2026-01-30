
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


use crate::db_tables::Network;
use crate::network::{Device, ToDeviceVector};


mod band;
mod types;


use band::add_devices_bands;
use types::device::{AsusDevice, NetworkMap, NetworkMapValue};


async fn get_devices_raw_data(asus_token: &String, network_gateway: &String) -> Option<String>
{
	let client = Client::new();
	let response: Response = client
	.get(format!("http://{}/update_clients.asp", network_gateway))
	.header("Cookie", asus_token)
	.header("Referer", format!("http://{}/device-map/clients.asp", network_gateway))
	.send()
	.await
	.ok()?;

	return response.text().await.ok();
}


fn parse_devices_raw_data(raw_data: String) -> Option<Vec<AsusDevice>>
{
	// FROM: https://doc.rust-lang.org/std/string/struct.String.html#method.split
	let lines: std::str::Split<char> = raw_data.split('\n');
	// FROM: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
	let line = lines.filter(|line| {line.starts_with("fromNetworkmapd : [")}).next()?;

	// FROM: https://doc.rust-lang.org/std/string/struct.String.html#method.strip_suffix
	let json = line.strip_prefix("fromNetworkmapd : [")?.strip_suffix("],")?;
	let fromNetworkmapd: NetworkMap = serde_json::from_str::<NetworkMap>(json).ok()?;

	return Some(
		fromNetworkmapd
		.into_values()
		.filter_map(
			|value: NetworkMapValue|
			{
				match(value)
				{
					NetworkMapValue::AsusDevice(value) => (value.isOnline == "1").then_some(value),
					_ => None,
				}
			}
		)
		.collect::<Vec<AsusDevice>>()
	);
}


pub async fn get_devices(asus_token: &String, network: &Network) -> Option<Vec<Device>>
{
	let device_raw_data: String = get_devices_raw_data(asus_token, &network.gateway).await?;
	let asus_devices: Vec<AsusDevice> = parse_devices_raw_data(device_raw_data)?;

	let mut devices = asus_devices.to_device_vec();
	for device in devices.iter_mut()
	{
		device.network_id = network.id;
	}

	add_devices_bands(asus_token, &network.gateway, &mut devices).await;

	return Some(devices);
}
