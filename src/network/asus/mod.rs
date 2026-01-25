
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


use reqwest;


use crate::db_tables::Network;


type AsusToken = String;


async fn get_asus_token(network: Network) -> Option<String>
{
	let credentials = match(network.credentials)
	{
		Some(credentials) => credentials,
		None => return None
	};

	let client = reqwest::Client::new();
	let response = client.post("http://".to_string() + &network.gateway)
	.header("Referer", "http://".to_string() + &network.gateway + "Main_Login.asp")
	.form(&[("login_authorization", credentials)])
	.send()
	.await;

	let unwrapped_response: reqwest::Response = match(response)
	{
		Ok(unwrapped_response) => unwrapped_response,
		Err(_) => return None,
	};
	let set_cookie_str: Result<&str, _> = match(unwrapped_response.headers().get("Set-Cookie"))
	{
		Some(set_cookie_header) => set_cookie_header.to_str(),
		None => return None,
	};
	return match(set_cookie_str)
	{
		Ok(cookie_string) => Some(cookie_string.to_string().split(';').next().unwrap().to_string()),
		// {
		// 	let mut token = Some(cookie_string.to_string().split(';')[0]);
		// 	token.replace_range(token.find(';').unwrap_or(token.len()));
		// 	token.remove_matches("; HttpOnly;");
		// 	Some(token)
		// },
		Err(_) => None,
	}
}


// Get raw data
// Parse data
