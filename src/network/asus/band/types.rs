
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2026.01.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use std::collections::HashMap;


use serde::{Deserialize, Serialize};


pub type WiredClient = HashMap<String, Vec<String>>;
pub type WirelessClient = HashMap<String, WirelessBands>;


#[derive(Clone, Deserialize, Serialize)]
pub struct WirelessBands
{
	#[serde(rename = "2G")]
	pub _2g: Vec<String>,
	#[serde(rename = "5G")]
	pub _5g: Vec<String>,
}


impl WirelessBands
{
	pub fn new() -> WirelessBands
	{
		return WirelessBands {
			_2g: vec![],
			_5g: vec![],
		}
	}
}
