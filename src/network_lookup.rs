
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


// #![allow(non_snake_case)]
#![allow(unused_parens)]
// #![allow(non_camel_case_types)]


mod db_tables;
mod network;
mod query;
mod response;
mod routes;


use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::postgres::PgPool;


use crate::routes::api;


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let host: String = std::env::var("NETWORKLOOKUP_DB_HOST").unwrap();
	let user: String = std::env::var("NETWORKLOOKUP_DB_USER").unwrap();
	let password: String = std::env::var("NETWORKLOOKUP_DB_PASSWORD").unwrap();
	let db_name: &str = "NetworkLookup";

	let connection_str: String = format!("postgres://{}:{}@{}:5432/{}", user, password, host, db_name);
	let connection_pool: PgPool = PgPool::connect(&connection_str)
	.await
	.expect("Failed to create Postgres DB connection pool");

	HttpServer::new
	(
		move ||
		{
			App::new().app_data(web::Data::new(connection_pool.clone()))
			.route("/", web::get().to(routes::get_index))
			.route("/api", web::get().to(api::get_index))
			.route("/api/groups", web::get().to(api::groups::get_index))
			.route("/api/groups/{label}", web::get().to(api::groups::get_label))
			.route("/api/networks", web::get().to(api::networks::get_index))
			.route("/api/networks/{id}", web::get().to(api::networks::get_id))
			.route("/api/networks/{network_id}/devices", web::get().to(api::networks::devices::get_index))
			.route("/api/networks/{network_id}/devices/allowed", web::get().to(api::networks::devices::get_allowed))
			.route("/api/networks/{network_id}/devices/allowed/2.4GHz", web::post().to(api::networks::devices::post_allowed("2.4GHz")))
			.route("/api/networks/{network_id}/devices/allowed/5GHz", web::post().to(api::networks::devices::post_allowed("5GHz")))
			.route("/api/networks/{network_id}/devices/static", web::post().to(api::networks::devices::post_static))
			.route("/api/networks/{network_id}/devices/{device_id}", web::get().to(api::networks::devices::get_id))
			.route("/api/services", web::get().to(api::services::get_index))
			.route("/api/services/{id}", web::get().to(api::services::get_id))
			.wrap(Logger::default())
		}
	)
	.bind("0.0.0.0:443")?
	.run()
	.await
}
