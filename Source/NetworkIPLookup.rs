
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.04.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]


use serde_json;


mod IP;
mod Network;
mod Queries;
mod QueryError;
mod Responses;


use crate::Queries::
{
	SELECT_IP_by_Network_label_AND_IP_label,
	SELECT_IPs_by_Network_label_AND_Group_label
};
use crate::Responses::
{
	IP_query_to_response_JSON,
	IPs_query_to_response_JSON
};



fn main()
{
	let Network_label: String = "Home".to_string();
	let IP_label: String = "Bedroom-Curtain".to_string();
	let IP_query = SELECT_IP_by_Network_label_AND_IP_label(Network_label, IP_label);
	let response_body = IP_query_to_response_JSON(IP_query);
	println!("{}", response_body);


	let Network_label: String = "Home".to_string();
	let Group_label: String = "House".to_string();
	let IPs_query = SELECT_IPs_by_Network_label_AND_Group_label(Network_label, Group_label);
	let responses_body = IPs_query_to_response_JSON(IPs_query);
	println!("{}", responses_body);
}
