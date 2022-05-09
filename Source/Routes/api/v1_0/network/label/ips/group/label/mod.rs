
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.08                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use actix_web::{http::header::ContentType, HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::Query::{query_to_response, Queries::IP::SELECT_IPs_by_Network_label_AND_Group_label};


// `/api/v1.0/network/label/{network_label}/ips/group`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{network_label}/ips/group/label/{group_label}": "List all IPs based on group label and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/ips/group/label/{group_label}`
pub async fn label(pool: web::Data<(PgPool)>, path: web::Path<(String, String)>) -> HttpResponse
{
	let (Network_label, Group_label) = path.into_inner();
	let query_response = SELECT_IPs_by_Network_label_AND_Group_label(pool.as_ref(), Network_label, Group_label).await;
	return query_to_response(query_response);
}