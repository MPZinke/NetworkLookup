
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


use crate::db_tables::{DBDevice, ToDBDeviceUpdateString, Network};
use crate::network::Device;


mod allowed_devices;
mod connected_devices;
mod static_devices;


use allowed_devices::set_allowed_devices;
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


pub async fn get_network_devices(network: &Network) -> Option<Vec<Device>>
{
	if(network.credentials.is_none())
	{
		return None;
	}

	let asus_token: String = get_asus_token(network).await?;
	return get_devices(&asus_token, network).await;
}


pub async fn update_allowed_devices(db_devices: &Vec<DBDevice>, network: &Network) -> bool
{
	let asus_token: String = match(get_asus_token(network).await)
	{
		Some(asus_token) => asus_token,
		None => return false,
	};

	let _2ghz_allowed_list_string: String = db_devices.allowed_list_string("2.4GHz");
	let _5ghz_allowed_list_string: String = db_devices.allowed_list_string("5GHz");

	let _2ghz_update_callback = set_allowed_devices(&asus_token, &network.gateway, "2.4GHz", &_2ghz_allowed_list_string);
	let _5ghz_update_callback = set_allowed_devices(&asus_token, &network.gateway, "5GHz", &_5ghz_allowed_list_string);

	return _2ghz_update_callback.await && _5ghz_update_callback.await;
}


pub async fn update_static_devices(db_devices: &Vec<DBDevice>, network: &Network) -> bool
{
	let asus_token: String = match(get_asus_token(network).await)
	{
		Some(asus_token) => asus_token,
		None => return false,
	};

	let static_list_string: String = db_devices.static_list_string();
	return set_static_devices(&asus_token, &network.gateway, &static_list_string).await;
}
