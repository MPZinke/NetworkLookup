
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
use crate::query::group::{SELECT_Groups_by_Device_address, SELECT_Groups_by_Device_id, SELECT_Groups_by_Device_label};


pub async fn SELECT_Device_by_Network_id_AND_Device_address(
	pool: &PgPool,
	Network_id: i32,
	Device_address: &String
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
		WHERE "Networks"."id" = $1 AND "Devices"."address" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id)
		.bind(Device_address.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!(
			"No results found for `Network`.`id`: '{}', `Device`.`address`: '{}'",
			Network_id,
			Device_address
		);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_address(pool, &Device_address).await?;
	let network = Network::new(
		result[0].get("Networks.id"),
		result[0].get("Networks.label"),
		result[0].get("Networks.gateway"),
		result[0].get("Networks.netmask")
	);
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn SELECT_Device_by_Network_label_AND_Device_address(
	pool: &PgPool,
	Network_label: &String,
	Device_address: &String
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
		WHERE "Networks"."label" = $1
			AND "Devices"."address" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Device_address.clone())
		.fetch_all(pool).await?;
	if(result.len() < 1)
	{
		let message: String = format!(
			"No results found for `Network`.`label`: '{}', `Device`.`address`: '{}'",
			Network_label,
			Device_address
		);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_address(pool, &Device_address).await?;
	let network = Network::new(
		result[0].get("Networks.id"),
		result[0].get("Networks.label"),
		result[0].get("Networks.gateway"),
		result[0].get("Networks.netmask")
	);
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn SELECT_Device_by_Network_id_AND_Device_id(
	pool: &PgPool,
	Network_id: i32,
	Device_id: i32
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
		.bind(Network_id.clone())
		.bind(Device_id.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!(
			"No results found for `Network`.`id`: '{}', `Device`.`id`: '{}'",
			Network_id,
			Device_id
		);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_id(pool, Device_id).await?;
	let network = Network::new(
		result[0].get("Networks.id"),
		result[0].get("Networks.label"),
		result[0].get("Networks.gateway"),
		result[0].get("Networks.netmask")
	);
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn SELECT_Device_by_Network_id_AND_Device_label(
	pool: &PgPool,
	Network_id: i32,
	Device_label: &String
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
			AND "Devices"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id.clone())
		.bind(Device_label.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!(
			"No results found for `Network`.`id`: '{}', `Device`.`label`: '{}'",
			Network_id,
			Device_label
		);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
	let network = Network::new(
		result[0].get("Networks.id"),
		result[0].get("Networks.label"),
		result[0].get("Networks.gateway"),
		result[0].get("Networks.netmask")
	);
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn SELECT_Device_by_Network_label_AND_Device_id(
	pool: &PgPool,
	Network_label: &String,
	Device_id: i32
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
		WHERE "Networks"."label" = $1
			AND "Devices"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Device_id.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!(
			"No results found for `Network`.`label`: '{}', `Device`.`id`: '{}'",
			Network_label,
			Device_id
		);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_id(pool, Device_id).await?;
	let network = Network::new(
		result[0].get("Networks.id"),
		result[0].get("Networks.label"),
		result[0].get("Networks.gateway"),
		result[0].get("Networks.netmask")
	);
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn SELECT_Device_by_Network_label_AND_Device_label(
	pool: &PgPool,
	Network_label: &String,
	Device_label: &String
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
		WHERE "Networks"."label" = $1
			AND "Devices"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Device_label.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!(
			"No results found for `Network`.`label`: '{}', `Device`.`label`: '{}'",
			Network_label,
			Device_label
		);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
	let network = Network::new(
		result[0].get("Networks.id"),
		result[0].get("Networks.label"),
		result[0].get("Networks.gateway"),
		result[0].get("Networks.netmask")
	);
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn SELECT_Devices_by_Network_id(
	pool: &PgPool,
	Network_id: i32
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
	let result: Vec<PgRow> = query(query_str).bind(Network_id).fetch_all(pool).await?;
	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
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


pub async fn SELECT_Devices_by_Network_label(
	pool: &PgPool,
	Network_label: &String) -> Result<Vec<Device>, LookupError>
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
		WHERE "Networks"."label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(Network_label.clone()).fetch_all(pool).await?;
	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
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


pub async fn SELECT_Devices_by_Network_id_AND_Group_id(
	pool: &PgPool,
	Network_id: i32,
	Group_id: i32
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			"Networks"."id" AS "Networks.id",
			"Networks"."label" AS "Networks.label",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		FROM "Groups-Devices"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Networks"."id" = $1
			AND "Groups"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id.clone())
		.bind(Group_id.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Devices.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
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


pub async fn SELECT_Devices_by_Network_id_AND_Group_label(
	pool: &PgPool,
	Network_id: i32,
	Group_label: &String
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
	SELECT
			"Devices".*,
			"Networks"."id" AS "Networks.id",
			"Networks"."label" AS "Networks.label",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Networks"."id" = $1
			AND "Groups"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id.clone())
		.bind(Group_label.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Devices.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
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


pub async fn SELECT_Devices_by_Network_label_AND_Group_id(
	pool: &PgPool,
	Network_label: &String,
	Group_id: i32
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			"Networks"."id" AS "Networks.id",
			"Networks"."label" AS "Networks.label",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		FROM "Groups-Devices"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Networks"."label" = $1
			AND "Groups"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Group_id.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Devices.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
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


pub async fn SELECT_Devices_by_Network_label_AND_Group_label(
	pool: &PgPool,
	Network_label: &String,
	Group_label: &String
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			"Networks"."id" AS "Networks.id",
			"Networks"."label" AS "Networks.label",
			"Networks"."gateway" AS "Networks.gateway",
			"Networks"."netmask" AS "Networks.netmask"
		FROM "Groups-Devices"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		JOIN "Networks" ON "Devices"."Networks.id" = "Networks"."id"
		WHERE "Networks"."label" = $1
			AND "Groups"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Group_label.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Devices.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
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
