
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.09.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use sqlx::{query, PgPool, postgres::PgRow, Row};


use crate::db_tables::Service;
use crate::lookup_error::LookupError;
use crate::query::devices::get_device_by_network_id_and_device_id;


pub async fn get_services(pool: &PgPool) -> Result<Vec<Service>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Services".*, "Devices"."id" as "Devices.id", "Networks"."id" as "Networks.id"
		FROM "Services"
		JOIN "Devices" ON "Services"."Devices.id" = "Devices"."id"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut services: Vec<Service> = vec![];
	for row in result
	{
		let device = get_device_by_network_id_and_device_id(pool, row.get("Networks.id"), row.get("Devices.id")).await?;
		services.push(Service::new(device, &row));
	}

	return Ok(services);
}


pub async fn get_service_by_id(pool: &PgPool, id: i32) -> Result<Vec<Service>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Services".*, "Devices"."id" as "Devices.id", "Networks"."id" as "Networks.id"
		FROM "Services"
		JOIN "Devices" ON "Services"."Devices.id" = "Devices"."id"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Services"."id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(id)
		.fetch_all(pool).await?;

	let mut services: Vec<Service> = vec![];
	for row in result
	{
		let device = get_device_by_network_id_and_device_id(pool, row.get("Networks.id"), row.get("Devices.id")).await?;
		services.push(Service::new(device, &row));
	}

	return Ok(services);
}
