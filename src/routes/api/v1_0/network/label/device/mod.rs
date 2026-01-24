
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.07                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod address;
pub mod id;
pub mod label;


use actix_web::{http::header::ContentType, HttpResponse};


// `/api/v1.0/networks/label/{label}/device`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{	
		"/api/v1.0/network/label/{network_label}/device/address": "Queries for device based on device address and network label",
		"/api/v1.0/network/label/{network_label}/device/id": "Queries for device based on device id and network label",
		"/api/v1.0/network/label/{network_label}/device/label": "Queries for device based on device label and network label",
		"/api/v1.0/network/label/{network_label}/devices/group": "Queries for devices based on group and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
