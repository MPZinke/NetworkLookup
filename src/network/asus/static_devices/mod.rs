
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


pub async fn set_static_devices(asus_token: &String, network_gateway: &String, static_list_string: &String) -> bool
{
	let client = Client::new();
	let response_result: Result<Response, Error> = client.post(format!("http://{}/start_apply.htm", network_gateway))
	.header("Referer", format!("http://{}/Advanced_DHCP_Content.asp", network_gateway))
	.header("Cookie", asus_token)
	.form(&[("action_mode", "apply_new"), ("dhcp_staticlist", static_list_string)])
	.send()
	.await;

	return match(response_result)
	{
		Ok(response) => response.status().is_success(),
		Err(_) => false,
	}
}
