
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.21                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use regex::Regex;


use crate::db_tables::device::Device;
use crate::search_type::{DeviceAttributeSearch, NetworkSearch};
use crate::unknown_lookup::{regex_and_default_to_empty_string, networks::NetworkInterface};


pub struct Netgear;


fn build_device_expression(device: &DeviceAttributeSearch) -> String
{
	let mut address_attribute: String = r"([0-9]{1,3}.){3}[0-9]{1,3}".to_string();
	let mut label_attribute: String = r"[\(\)_\s*\-:#&a-zA-Z0-9]*".to_string();
	let mut mac_attribute: String = r"([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}".to_string();

	match(device)
	{
		DeviceAttributeSearch::address(address) => address_attribute = address.clone(),
		DeviceAttributeSearch::label(label) => label_attribute = label.clone(),
		DeviceAttributeSearch::mac(mac) => mac_attribute = mac.clone()
	}

	return format!(
	  concat!(r#"<tr><td\s*align="center"><input\s*name="check_dev"\s*type="checkbox"\s*value="{}"onclick="handle_ch"#,
	    r#"eckboxElements\(this\);"></td>\s*\s*\s*<td\s*align="center"\s*name="show_status"><span\s*class="clickMe"\"#,
	    r#"s*>[a-zA-Z]*</span></td>\s*\s*\s*<td\s*align="center"><span\s*class="clickMe"\s*>(2\.4G\s*Wireless|5G\s*W"#,
	    r#"ireless|Wired)</span></td>\s*\s*\s*<td\s*align="center"><span\s*class="clickMe"\s*\s*><table\s*width="100"#,
	    r#"%"\s*title="[\(\)_\s*\-:#&a-zA-Z0-9]+"><tr><td><img\s*width=40px\s*height=40px\s*src="[_/\.a-zA-Z0-9]+"><"#,
	    r#"/td><td\s*align="right"><span>[\(\)_\s*\-:#&a-zA-Z0-9]*<br>{}</span></td></tr></table></span></td>\s*<td\"#,
	    r#"s*align="center"><span\s*class="clickMe"\s*>{}</span></td>\s*\s*\s*<td\s*align="center"><span\s*class="cl"#,
	    r#"ickMe"\s*name="mac"\s*>{}</span></td>"#),
	  mac_attribute, label_attribute, address_attribute, mac_attribute);
}


impl NetworkInterface for Netgear
{
	const ATTACHED_DEVICES_PATH: &'static str = "DEV_device_dev_id.htm";


	fn parse_response_to_section(device: &DeviceAttributeSearch, response: &String) -> Option<String>
	{
		let expression: String = build_device_expression(device);

		let regex: Regex = match(Regex::new(&expression))
		{
			Ok(regex) => regex,
			Err(_) => return None
		};

		match(regex.find(&response))
		{
			None => return None,
			Some(match_value) => return Some(match_value.as_str().to_string())
		}
	}


	fn parse_section_to_device(network: &NetworkSearch, section: &String) -> Device
	{
		let address_regex: String = r"([0-9]{1,3}.){3}[0-9]{1,3}".to_string();
		let address: String = regex_and_default_to_empty_string(&address_regex, section);
		let label_regex: String = r"<br>[\(\)_ \-:#&a-zA-Z0-9]*</span>".to_string();
		let label_section: String = regex_and_default_to_empty_string(&label_regex, section);
		let label: String = label_section[4..label_section.len()-7].to_string();
		let mac_regex: String = r"([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}".to_string();
		let mac: String = regex_and_default_to_empty_string(&mac_regex, section);

		return Device{id: -1, address: Some(address), label: label, is_reservation: false, is_static: false,
		  mac: Some(mac), groups: vec![], Network: network.network().clone()};
	}	
}


