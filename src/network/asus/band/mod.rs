
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
use crate::network::Device;


mod types;


use types::{WiredClient, WirelessClient, WirelessBands};


async fn get_devices_bands_raw_data(asus_token: String, network_gateway: &String) -> Option<String>
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


fn parse_devices_bands_raw_data(raw_data: String, devices: Vec<Device>) -> ()
{
	let mut wireless_bands: Option<WirelessBands> = None;
	let mut wired_bands: Option<Vec<String>> = None;

	let lines: std::str::Split<char> = raw_data.split('\n');
	for line in lines
	{
		if(line.starts_with("get_wclientlist = ["))
		{
			let json = line.strip_prefix("get_wclientlist = [")?.strip_suffix("][0];")?;  // TODO: Remove ? operators
			wireless_bands = match(serde_json::from_str::<WirelessClient>(json))
			{
				Ok(wireless_client) => wireless_client.into_values().next(),
				Err(_) => None,
			}
		}
		else if(line.starts_with("get_wiredclientlist = ["))
		{
			let json = line.strip_prefix("get_wiredclientlist = [")?.strip_suffix("][0];")?;  // TODO: Remove ? operators
			wired_bands = match(serde_json::from_str::<WiredClient>(json))
			{
				Ok(wired_client) => wired_client.into_values().next(),
				Err(_) => None,
			}
		}
	}

	// if(wireless_bands.is_some())
	// {
	// 	for mut device in devices
	// 	{
	// 		if(wireless_bands.unwrap()._2g.contains(&device.mac))
	// 		{
	// 			device.band = Some(String::from("2.4GHz"));
	// 		}
	// 	}
	// }


	Some(())
}
