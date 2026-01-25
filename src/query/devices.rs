
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


use sqlx::{query, PgPool, postgres::PgRow, Row};


use crate::db_tables::{Device, Group, Network};
use crate::lookup_error::{LookupError, NewNotFoundError};
use crate::query::groups::get_groups_by_device_id;


pub async fn get_device_by_network_id_and_device_id(
	pool: &PgPool,
	network_id: i32,
	device_id: i32
) -> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			"Networks"."id" AS "Networks.id",
			"Networks"."label" AS "Networks.label",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		FROM "Devices"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Networks"."id" = $1
			AND "Devices"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(network_id.clone())
		.bind(device_id.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!(
			"No results found for `Network`.`id`: '{}', `Device`.`id`: '{}'",
			network_id,
			device_id
		);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = get_groups_by_device_id(pool, device_id).await?;
	let network = Network::new(
		result[0].get("Networks.id"),
		result[0].get("Networks.label"),
		result[0].get("Networks.gateway"),
		result[0].get("Networks.netmask")
	);
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn get_devices_by_network_id(
	pool: &PgPool,
	network_id: i32
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			"Networks"."id" AS "Networks.id",
			"Networks"."label" AS "Networks.label",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		FROM "Devices"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Networks"."id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(network_id).fetch_all(pool).await?;
	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let groups: Vec<Group> = get_groups_by_device_id(pool, row.get("id")).await?;
		let network = Network::new(
			row.get("Networks.id"),
			row.get("Networks.label"),
			row.get("Networks.gateway"),
			row.get("Networks.netmask")
		);
		devices.push(Device::new(groups, network, &row));
	}

	return Ok(devices);
}
