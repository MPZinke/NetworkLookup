
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2026.01.24                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use reqwest::{Client, Response};


use crate::db_tables::Network;
use crate::network::{Device, ToDeviceVector};


mod types;


use types::{AsusDevice, AsusToken, NetworkMap, NetworkMapValue};


async fn get_asus_token(network: &Network) -> Option<AsusToken>
{
	let credentials = match(&network.credentials)
	{
		Some(credentials) => credentials,
		None => return None,
	};

	// let client = Client::builder().cookie_store(true).build().ok()?;
	let client = Client::new();
	let response = client.post(format!("http://{}/login.cgi", network.gateway))
	.header("Referer", format!("http://{}/Main_Login.asp", network.gateway))
	.form(&[("login_authorization", credentials)])
	.send()
	.await;

	let unwrapped_response: reqwest::Response = response.ok()?;
	let set_cookie_str_result: Result<&str, _> = unwrapped_response.headers().get("Set-Cookie")?.to_str();
	return match(set_cookie_str_result)
	{
		Ok(cookie_str) => Some(cookie_str.split(';').next().unwrap().to_string()),
		Err(_) => None,
	};
}


async fn get_raw_data(network: &Network) -> Option<String>
{
	let asus_token: AsusToken = get_asus_token(network).await?;
	println!("Got asus token {}", asus_token);

	let client = Client::new();
	let response: Response = client
	.get(format!("http://{}/update_clients.asp", network.gateway))
	.header("Cookie", asus_token)
	.header("Referer", format!("http://{}/device-map/clients.asp", network.gateway))
	.send()
	.await
	.ok()?;

	return response.text().await.ok();
}


fn parse_raw_data(raw_data: String) -> Option<Vec<AsusDevice>>
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
					NetworkMapValue::AsusDevice(value) => Some(value),
					_ => None,
				}
			}
		)
		.collect::<Vec<AsusDevice>>()
	);
}


pub async fn get_devices(network: &Network) -> Option<Vec<Device>>
{
	let raw_data: String = get_raw_data(network).await?;
	let asus_devices: Vec<AsusDevice> = parse_raw_data(raw_data)?;

	return Some(asus_devices.to());
}
