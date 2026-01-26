
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


use std::collections::HashMap;


use serde::{Deserialize, Serialize};
use reqwest::{Client, Response};


use crate::db_tables::Network;


#[derive(Deserialize, Serialize)]
pub struct AsusDevice
{
	ip: String,
	name: String,
	nickName: String,
	mac: String,
}
pub type AsusDevices = HashMap<String, AsusDevice>;
type AsusToken = String;


async fn get_asus_token(network: &Network) -> Option<AsusToken>
{
	// let credentials = (&network.credentials)?;
	let credentials = match(&network.credentials)
	{
		Some(credentials) => credentials,
		None => return None,
	};

	let client = Client::new();
	let response = client.post(format!("http://{}", network.gateway))
	.header("Referer", format!("http://{}/Main_Login.asp", network.gateway) /*"http://".to_string() + &network.gateway + "/Main_Login.asp"*/)
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


fn parse_raw_data(raw_data: String) -> Option<AsusDevices>
{
	// FROM: https://doc.rust-lang.org/std/string/struct.String.html#method.split
	let lines: std::str::Split<char> = raw_data.split('\n');
	// FROM: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
	let line = match(lines.filter(|line| {line.starts_with("fromNetworkmapd : [")}).next())
	{
		Some(line) => line,
		None => return None,
	};

	// FROM: https://doc.rust-lang.org/std/string/struct.String.html#method.strip_suffix
	// FROM: https://doc.rust-lang.org/std/string/struct.String.html#method.trim_prefix
	let json = line.strip_prefix("fromNetworkmapd : [")?.strip_suffix("],")?;

	return Some(serde_json::from_str(json).ok()?);
}


pub async fn get_devices(network: &Network) -> Option<Vec<String>>
{
	let raw_data: String = get_raw_data(network).await?;

	return vec![].into();
}
