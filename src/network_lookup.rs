
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
mod lookup_error;
mod query;
mod routes;


use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;


use crate::routes::api;


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let host: &str = "localhost";
	let user: &str = env!("NETWORKLOOKUP_DB_USER");
	let password: &str = env!("NETWORKLOOKUP_DB_PASSWORD");
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
			.route("/", web::get().to(routes::index))
			.route("/api", web::get().to(api::index))
			.route("/api/groups", web::get().to(api::groups::index))
			.route("/api/groups/{id}", web::get().to(api::groups::id))
			.route("/api/networks", web::get().to(api::networks::index))
			.route("/api/networks/{id}", web::get().to(api::networks::id))
			.route("/api/networks/{network_id}/devices", web::get().to(api::networks::devices::index))
			.route("/api/networks/{network_id}/devices/{device_id}", web::get().to(api::networks::devices::id))
			.route("/api/services", web::get().to(api::services::index))
			.route("/api/services/{id}", web::get().to(api::services::id))
		}
	)
	.bind("0.0.0.0:443")?
	.run()
	.await
}
