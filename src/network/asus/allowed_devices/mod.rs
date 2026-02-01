
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2026.01.31                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use reqwest::{Client, Error, Response};


pub async fn set_allowed_devices(
	asus_token: &String,
	network_gateway: &String,
	band: &str,
	allowed_list_string: &String
) -> bool
{
	let wireless_unit: &str = match(band)
	{
		"2.4GHz" => "0",
		"5GHz" => "1",
		_ => return false,
	};

	let client = Client::new();
	let response_result: Result<Response, Error> = client.post(format!("http://{}/start_apply.htm", network_gateway))
	.header("Referer", format!("http://{}/Advanced_DHCP_Content.asp", network_gateway))
	.header("Cookie", asus_token)
	.form(
		&[
			("action_mode", "apply_new"),
			("wl_maclist_x", allowed_list_string),
			("wl_macmode", "allow"),
			("wl_unit", wireless_unit),
		]
	)
	.send()
	.await;

	return match(response_result)
	{
		Ok(response) => response.status().is_success(),
		Err(error) => {println!("{}", error); false},
	}
}
