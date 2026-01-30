
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


mod band;
mod devices;


use devices::{get_bandless_devices, AsusDevice, NetworkMap, NetworkMapValue};


async fn get_asus_token(network: &Network) -> Option<String>
{
	// let client = Client::builder().cookie_store(true).build().ok()?;
	let client = Client::new();
	let response = client.post(format!("http://{}/login.cgi", network.gateway))
	.header("Referer", format!("http://{}/Main_Login.asp", network.gateway))
	.form(&[("login_authorization", &network.credentials)])
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


pub async fn get_network_devices(network: &Network) -> Option<Vec<Device>>
{
	if(network.credentials.is_none())
	{
		return None;
	}

	let asus_token: String = get_asus_token(network).await?;
	let bandless_devices: Option<Vec<Device>> = get_bandless_devices(asus_token, network).await;
	return bandless_devices;
}
