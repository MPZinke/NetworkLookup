
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


use actix_web::{web, App, HttpServer, Responder};


mod IP;
mod Network;
mod Queries;
mod QueryError;
mod Routes;


use crate::Routes::api;


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	std::env::set_var("RUST_LOG", "actix_web=info");

	HttpServer::new
	(
		||
		{
			App::new()
			  .route("/", web::get().to(Routes::index))
			  .route("/api", web::get().to(api::index))

			  .route("/api/v1.0", web::get().to(api::v1_0::index))
			  .route("/api/v1.0/network", web::get().to(api::v1_0::network::index))
			  .route("/api/v1.0/network/label/{label}", web::get().to(api::v1_0::network::label::index))
			  .route("/api/v1.0/network/label/{label}/ip", web::get().to(api::v1_0::network::label::ip::index))
		}
	)
	  .bind("127.0.0.1:8080")?
	  .run()
	  .await
}
