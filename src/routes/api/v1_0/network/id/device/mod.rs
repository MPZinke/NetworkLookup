
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


// `/api/v1.0/network/id/{network_id}/device`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/id/{network_id}/device/address": "Queries for device based on device address and network id",
		"/api/v1.0/network/id/{network_id}/device/id": "Queries for device based on device id and network id",
		"/api/v1.0/network/id/{network_id}/device/label": "Queries for device based on device label and network id",
		"/api/v1.0/network/id/{network_id}/devices/group": "Queries for devices based on group and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
