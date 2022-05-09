
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.09                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use actix_web::{http::header::ContentType, HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::Queries::{query_to_json, SELECT_IP_by_Network_label_AND_IP_address};


pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}/ip/address/{address}": "Get an IP by IP address and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


pub async fn address(pool: web::Data<(PgPool)>, path: web::Path<(String, String)>) -> HttpResponse
{
	let (Network_label, IP_address) = path.into_inner();
	let query_response = SELECT_IP_by_Network_label_AND_IP_address(pool.as_ref(), Network_label, IP_address).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
