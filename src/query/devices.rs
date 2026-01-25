
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
use crate::lookup_error::{LookupError, new_not_found_error};
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
			"Networks"."credentials" AS "Networks.credentials",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		FROM "Devices"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Networks"."id" = $1
			AND "Devices"."id" = $2;
	"#;
	let result: sqlx::Result<PgRow> = query(query_str).bind(network_id).bind(device_id).fetch_one(pool).await;
	let row = match(result)
	{
		Ok(row) => row,
		Err(_)
		=>
		{
			let message: String = format!(
				"No results found for `Network`.`id`: '{}', `Device`.`id`: '{}'",
				network_id,
				device_id
			);
			return Err(new_not_found_error(message));
		}
	};

	let groups: Vec<Group> = get_groups_by_device_id(pool, device_id).await?;
	let network = Network {
		id: row.get("Networks.id"),
		credentials: row.get("Networks.credentials"),
		label: row.get("Networks.label"),
		gateway: row.get("Networks.gateway"),
		netmask: row.get("Networks.netmask")
	};
	return Ok(Device::new(&row, groups, network));
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
			"Networks"."credentials" AS "Networks.credentials",
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
		let network = Network {
			id: row.get("Networks.id"),
			credentials: row.get("Networks.credentials"),
			label: row.get("Networks.label"),
			gateway: row.get("Networks.gateway"),
			netmask: row.get("Networks.netmask")
		};
		devices.push(Device::new(&row, groups, network));
	}

	return Ok(devices);
}



pub async fn get_devices_by_group_label(
	pool: &PgPool,
	group_label: String
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			"Networks"."id" AS "Networks.id",
			"Networks"."label" AS "Networks.label",
			"Networks"."credentials" AS "Networks.credentials",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		FROM "Groups-Devices"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Groups"."label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(&group_label).fetch_all(pool).await?;
	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let groups: Vec<Group> = vec![group_label.clone()];
		let network = Network {
			id: row.get("Networks.id"),
			credentials: row.get("Networks.credentials"),
			label: row.get("Networks.label"),
			gateway: row.get("Networks.gateway"),
			netmask: row.get("Networks.netmask")
		};
		devices.push(Device::new(&row, groups, network));
	}

	return Ok(devices);
}
