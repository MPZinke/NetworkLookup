
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.05                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod ip;


use actix_web::{http::header::ContentType, HttpResponse, web};


use crate::Queries::{generic_query_to_response_JSON, SELECT_Networks_by_label};


// `/api/v1.0/network/label/{label}`
pub async fn index(path: web::Path<(String)>) -> HttpResponse
{
	let (label) = path.into_inner();
	let query_response = SELECT_Networks_by_label(label);
	let body = generic_query_to_response_JSON(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}