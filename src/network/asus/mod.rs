
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


use reqwest::Client;


use crate::db_tables::{DBDevice, Network};
use crate::network::Device;


mod connected_devices;
mod static_devices;


use connected_devices::get_devices;
use static_devices::set_static_devices;


async fn get_asus_token(network: &Network) -> Option<String>
{
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


pub async fn get_asus_network_devices(network: &Network) -> Option<Vec<Device>>
{
	if(network.credentials.is_none())
	{
		return None;
	}

	let asus_token: String = get_asus_token(network).await?;
	return get_devices(&asus_token, network).await;
}


pub async fn update_asus_static_devices(db_devices: &Vec<DBDevice>, network: &Network) -> bool
{
	let asus_token: String = match(get_asus_token(network).await)
	{
		Some(asus_token) => asus_token,
		None => return false,
	};

	let static_list_string: String = db_devices
	.iter()
	.map(|db_device|{db_device.to_static_list_string()})
	.collect::<Vec<String>>()
	.join("");

	return set_static_devices(&asus_token, &network.gateway, &static_list_string).await;
}
