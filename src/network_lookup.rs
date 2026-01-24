
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


mod db_tables;
mod lookup_error;
mod query;
mod routes;
mod search_type;
mod unknown_lookup;


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

	println!("Connecting to DB");
	let connection_str: String = format!("postgres://{}:{}@{}:5432/{}", user, password, host, db_name);
	let connection_pool: PgPool = PgPool::connect(&connection_str)
		.await
		.expect("Failed to create Postgres DB connection pool");

	println!("Initializing Server");
	HttpServer::new
	(
		move ||
		{
			App::new().app_data(web::Data::new(connection_pool.clone()))
			  .route("/", web::get().to(routes::index))
			  .route("/api", web::get().to(api::index))
			  .route("/api/v1.0", web::get().to(api::v1_0::index))
			  // —— GROUP —— //
			  .route("/api/v1.0/group", web::get().to(api::v1_0::group::index))
			  .route("/api/v1.0/group/all", web::get().to(api::v1_0::group::all))
			  .route("/api/v1.0/group/id", web::get().to(api::v1_0::group::id::index))
			  .route("/api/v1.0/group/id/{group_id}", web::get().to(api::v1_0::group::id::id))
			  .route("/api/v1.0/group/label", web::get().to(api::v1_0::group::label::index))
			  .route("/api/v1.0/group/label/{group_label}", web::get().to(api::v1_0::group::label::label))

			  .route("/api/v1.0/network", web::get().to(api::v1_0::network::index))
			  .route("/api/v1.0/network/all", web::get().to(api::v1_0::network::all))

			  .route("/api/v1.0/network/id", web::get().to(api::v1_0::network::id::index))
			  .route("/api/v1.0/network/id/{network_id}", web::get().to(api::v1_0::network::id::id))
			  .route("/api/v1.0/network/id/{network_id}/device", web::get().to(api::v1_0::network::id::device::index))
			  .route("/api/v1.0/network/id/{network_id}/device/address", web::get().to(api::v1_0::network::id::device::address::index))
			  .route("/api/v1.0/network/id/{network_id}/device/address/{device_address}", web::get().to(api::v1_0::network::id::device::address::address))
			  .route("/api/v1.0/network/id/{network_id}/device/id", web::get().to(api::v1_0::network::id::device::id::index))
			  .route("/api/v1.0/network/id/{network_id}/device/id/{device_id}", web::get().to(api::v1_0::network::id::device::id::id))
			  .route("/api/v1.0/network/id/{network_id}/device/label", web::get().to(api::v1_0::network::id::device::label::index))
			  .route("/api/v1.0/network/id/{network_id}/device/label/{device_label}", web::get().to(api::v1_0::network::id::device::label::label))
			  .route("/api/v1.0/network/id/{network_id}/devices", web::get().to(api::v1_0::network::id::devices::index))
			  .route("/api/v1.0/network/id/{network_id}/devices/all", web::get().to(api::v1_0::network::id::devices::all))
			  .route("/api/v1.0/network/id/{network_id}/devices/group", web::get().to(api::v1_0::network::id::devices::group::index))
			  .route("/api/v1.0/network/id/{network_id}/devices/group/id", web::get().to(api::v1_0::network::id::devices::group::id::index))
			  .route("/api/v1.0/network/id/{network_id}/devices/group/id/{group_id}", web::get().to(api::v1_0::network::id::devices::group::id::id))
			  .route("/api/v1.0/network/id/{network_id}/devices/group/label", web::get().to(api::v1_0::network::id::devices::group::label::index))
			  .route("/api/v1.0/network/id/{network_id}/devices/group/label/{group_label}", web::get().to(api::v1_0::network::id::devices::group::label::label))
			  .route("/api/v1.0/network/id/{network_id}/services", web::get().to(api::v1_0::network::id::services::index))
			  .route("/api/v1.0/network/id/{network_id}/services/label", web::get().to(api::v1_0::network::id::services::label::index))
			  .route("/api/v1.0/network/id/{network_id}/services/label/{services_label}", web::get().to(api::v1_0::network::id::services::label::label))

			  .route("/api/v1.0/network/label", web::get().to(api::v1_0::network::label::index))
			  .route("/api/v1.0/network/label/{network_label}", web::get().to(api::v1_0::network::label::label))
			  .route("/api/v1.0/network/label/{network_label}/device", web::get().to(api::v1_0::network::label::device::index))
			  .route("/api/v1.0/network/label/{network_label}/device/address", web::get().to(api::v1_0::network::label::device::address::index))
			  .route("/api/v1.0/network/label/{network_label}/device/address/{device_address}", web::get().to(api::v1_0::network::label::device::address::address))
			  .route("/api/v1.0/network/label/{network_label}/device/id", web::get().to(api::v1_0::network::label::device::id::index))
			  .route("/api/v1.0/network/label/{network_label}/device/id/{device_id}", web::get().to(api::v1_0::network::label::device::id::id))
			  .route("/api/v1.0/network/label/{network_label}/device/label", web::get().to(api::v1_0::network::label::device::label::index))
			  .route("/api/v1.0/network/label/{network_label}/device/label/{device_label}", web::get().to(api::v1_0::network::label::device::label::label))
			  .route("/api/v1.0/network/label/{network_label}/devices", web::get().to(api::v1_0::network::label::devices::index))
			  .route("/api/v1.0/network/label/{network_label}/devices/all", web::get().to(api::v1_0::network::label::devices::all))
			  .route("/api/v1.0/network/label/{network_label}/devices/group", web::get().to(api::v1_0::network::label::devices::group::index))
			  .route("/api/v1.0/network/label/{network_label}/devices/group/id", web::get().to(api::v1_0::network::label::devices::group::id::index))
			  .route("/api/v1.0/network/label/{network_label}/devices/group/id/{group_id}", web::get().to(api::v1_0::network::label::devices::group::id::id))
			  .route("/api/v1.0/network/label/{network_label}/devices/group/label", web::get().to(api::v1_0::network::label::devices::group::label::index))
			  .route("/api/v1.0/network/label/{network_label}/devices/group/label/{group_label}", web::get().to(api::v1_0::network::label::devices::group::label::label))
			  .route("/api/v1.0/network/label/{network_label}/services", web::get().to(api::v1_0::network::label::services::index))
			  .route("/api/v1.0/network/label/{network_label}/services/label", web::get().to(api::v1_0::network::label::services::label::index))
			  .route("/api/v1.0/network/label/{network_label}/services/label/{services_label}", web::get().to(api::v1_0::network::label::services::label::label))
		}
	)
	  .bind("0.0.0.0:8081")?
	  .run()
	  .await
}
