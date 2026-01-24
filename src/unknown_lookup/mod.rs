
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.11                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod networks;


use regex::Regex;
use reqwest::header::AUTHORIZATION;


use crate::db_tables::Device;
use crate::lookup_error::{NewNotFoundError, LookupError};
use crate::search_type::{DeviceAttributeSearch, NetworkSearch};
use crate::unknown_lookup::networks::NetworkInterface;


fn device_not_found_error(device: &DeviceAttributeSearch, network: &NetworkSearch) -> LookupError
{
	let network_search_attritube: String = match(network)
	{
		NetworkSearch::id(network_value) => network_value.id.to_string(),
		NetworkSearch::label(network_value) => network_value.label.clone()
	};

	let body: String = format!("No results found for `Network`.`{}`: '{}', `Device`.`{}`: '{:?}'", network,
	  network_search_attritube, device, device);
	return NewNotFoundError(body);
}


/*
SUMMARY: Looks up a device on the network for a given NetworkInterface <N> (eg Netgear's Web Interface).
PARAMS:  Takes the device to search for, the network to search on, and the interface ( <N> ) to search with.
DETAILS: Makes a request to the router's page using <N>'s ATTACHED_DEVICES_PATH. If request was successful, the data
         from the page is sent to <N>'s devices expression builder. A string expression to Regex is returned. If the
         Regex is successful, the section containing the device's information is returned. The section is converted to a
         device.
RETURNS: Device if all processes successful. NotFound lookup_error if router request, regex, failed, etc.
*/
pub async fn lookup_device_on_network<N>(device_search: &DeviceAttributeSearch, network: &NetworkSearch)
  -> Result<Device, LookupError>
  where N: NetworkInterface
{
	let response: String = match(router_request(N::ATTACHED_DEVICES_PATH, &network).await)
	{
		Ok(response) => response,
		Err(error) => return Err(NewNotFoundError(error.to_string()))
	};

	let section: String = match(N::parse_response_to_section(device_search, &response))
	{
		Some(section) => section,
		None => return Err(device_not_found_error(device_search, network))
	};

	return Ok(N::parse_section_to_device(network, &section));
}


/*
SUMMARY: Helper function to automatically run a regex and return an empty String on any failure.
PARAMS:  Takes the expression to build the regex with, the section to run the regex on.
DETAILS: Attempts to build the Regex. If successful, runs the regex. On any failire returns an empty String.
RETURNS: The found regexed value if successful, otherwise an empty string.
*/
fn regex_and_default_to_empty_string(expression: &String, section: &String) -> String
{
	let empty_string: String = "".to_string();
	match(Regex::new(&expression))
	{
		Err(_) => return empty_string,
		Ok(regex)
		=>
		{
			match(regex.find(&section))
			{
				Some(match_value) => return match_value.as_str().to_string(),
				None => return empty_string
			}
		}
	}
}


/*
SUMMARY: 
PARAMS:  
DETAILS: 
RETURNS: 
*/
async fn router_request(attached_devices_path: &str, network: &NetworkSearch) -> Result<String, LookupError>
{
	let mut headers = reqwest::header::HeaderMap::new();
	// let auth_value = network.network().auth_value.as_ref().unwrap().as_ref();
	let auth_value = "";
	let header_value = reqwest::header::HeaderValue::from_str(auth_value)?;
	headers.insert(AUTHORIZATION, header_value);

	let client = reqwest::Client::builder().default_headers(headers).build()?;
	return Ok(client.get(format!("http://{}/{}", &network.network().gateway, attached_devices_path)).send().await?
	  .text().await?);
}

