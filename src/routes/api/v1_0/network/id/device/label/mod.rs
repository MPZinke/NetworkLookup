
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


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::db_tables::{device::Device, network::Network};
use crate::lookup_error::LookupError;
use crate::query::{query_NotFound, query_to_response};
use crate::query::{network::SELECT_Network_by_id, device::SELECT_Device_by_Network_id_AND_Device_label};
use crate::search_type::{DeviceAttributeSearch, NetworkSearch};
use crate::unknown_lookup::networks::lookup_device;


// `/api/v1.0/network/id/{network_id}/device/label`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/id/{network_id}/device/label/{device_label}": "Get a device by device label and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}/device/label/{device_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(i32, String)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_id, Device_label) = path.into_inner();
	let device_result: Result<Device, LookupError> = SELECT_Device_by_Network_id_AND_Device_label(pool.as_ref(),
	  Network_id, &Device_label).await;

	// If not found in DB, try to find Device label by scanning network
	if(query_NotFound(&device_result))
	{
		// Check and make sure Network exists
		let network_result: Result<Network, LookupError> = SELECT_Network_by_id(pool.as_ref(), Network_id).await;
		let network_search: NetworkSearch = match(network_result)
		{
			Ok(network_search) => NetworkSearch::id(network_search),
			Err(_) => return query_to_response(network_result)  // show both errors: DB errors need to be visible too
		};
	}

	return query_to_response(device_result);
}

