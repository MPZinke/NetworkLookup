
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


use crate::db_tables::{device::Device, group::Group};
use crate::lookup_error::{LookupError, NewNotFoundError};
use crate::query::group::{SELECT_Groups_by_Device_address, SELECT_Groups_by_Device_id, SELECT_Groups_by_Device_label};


pub async fn SELECT_Device_by_Network_id_AND_Device_address(pool: &PgPool, Network_id: i32, Device_address: &String)
-> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			"Network"."id" AS "Network.id",
			"Network"."label" AS "Network.label",
			"Network"."gateway" AS "Network.gateway",
			"Network"."netmask" AS "Network.netmask"
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
		row.get("Network.id"),
		row.get("Network.label"),
		row.get("gateway"),
		row.get("netmask")
	)
	return Ok(Device::new(groups, network, &result[0]));
}


pub async fn SELECT_Device_by_Network_label_AND_Device_address(pool: &PgPool, Network_label: &String, Device_address: &String)
	-> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device".*,
		"Network"."id" AS "Network.id",
		"Network"."label" AS "Network.label",
		"Network"."gateway",
		"Network"."netmask"
		FROM "Device"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."label" = $1
			AND "Device"."address" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Device_address.clone())
		.fetch_all(pool).await?;
	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`label`: '{}', `Device`.`address`: '{}'",
			Network_label, Device_address);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_address(pool, &Device_address).await?;
	return Ok(Device::new(groups, &result[0]));
}


pub async fn SELECT_Device_by_Network_id_AND_Device_id(pool: &PgPool, Network_id: i32, Device_id: i32)
	-> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Device"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."id" = $1
			AND "Device"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id.clone())
		.bind(Device_id.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`id`: '{}', `Device`.`id`: '{}'", Network_id, Device_id);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_id(pool, Device_id).await?;
	return Ok(Device::new(groups, &result[0]));
}


pub async fn SELECT_Device_by_Network_id_AND_Device_label(pool: &PgPool, Network_id: i32, Device_label: &String)
	-> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Device"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."id" = $1
			AND "Device"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id.clone())
		.bind(Device_label.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`id`: '{}', `Device`.`label`: '{}'", Network_id,
			Device_label);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
	return Ok(Device::new(groups, &result[0]));
}


pub async fn SELECT_Device_by_Network_label_AND_Device_id(pool: &PgPool, Network_label: &String, Device_id: i32)
	-> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Device"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."label" = $1
			AND "Device"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Device_id.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`label`: '{}', `Device`.`id`: '{}'", Network_label,
			Device_id);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_id(pool, Device_id).await?;
	return Ok(Device::new(groups, &result[0]));
}


pub async fn SELECT_Device_by_Network_label_AND_Device_label(pool: &PgPool, Network_label: &String, Device_label: &String)
	-> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Device"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."label" = $1
			AND "Device"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Device_label.clone())
		.fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`label`: '{}', `Device`.`label`: '{}'", Network_label,
			Device_label);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
	return Ok(Device::new(groups, &result[0]));
}


pub async fn SELECT_Devices_by_Network_id(pool: &PgPool, Network_id: i32) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Device"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(Network_id).fetch_all(pool).await?;
	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
		devices.push(Device::new(groups, &row));
	}

	return Ok(devices);
}


pub async fn SELECT_Devices_by_Network_label(pool: &PgPool, Network_label: &String) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Device"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(Network_label.clone()).fetch_all(pool).await?;
	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
		devices.push(Device::new(groups, &row));
	}

	return Ok(devices);
}


pub async fn SELECT_Devices_by_Network_id_AND_Group_id(pool: &PgPool, Network_id: i32, Group_id: i32)
	-> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Group-Device"
		JOIN "Device" ON "Group-Device"."Device.id" = "Device"."id"
		JOIN "Group" ON "Group-Device"."Group.id" = "Group"."id"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."id" = $1
			AND "Group"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id.clone())
		.bind(Group_id.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Device.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
		devices.push(Device::new(groups, &row));
	}

	return Ok(devices);
}


pub async fn SELECT_Devices_by_Network_id_AND_Group_label(pool: &PgPool, Network_id: i32, Group_label: &String)
	-> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Group-Device"
		JOIN "Device" ON "Group-Device"."Device.id" = "Device"."id"
		JOIN "Group" ON "Group-Device"."Group.id" = "Group"."id"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."id" = $1
			AND "Group"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_id.clone())
		.bind(Group_label.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Device.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
		devices.push(Device::new(groups, &row));
	}

	return Ok(devices);
}


pub async fn SELECT_Devices_by_Network_label_AND_Group_id(pool: &PgPool, Network_label: &String, Group_id: i32)
	-> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Group-Device"
		JOIN "Device" ON "Group-Device"."Device.id" = "Device"."id"
		JOIN "Group" ON "Group-Device"."Group.id" = "Group"."id"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."label" = $1
			AND "Group"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Group_id.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Device.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
		devices.push(Device::new(groups, &row));
	}

	return Ok(devices);
}


pub async fn SELECT_Devices_by_Network_label_AND_Group_label(pool: &PgPool, Network_label: &String, Group_label: &String)
	-> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label",
			"Device"."is_reservation", "Device"."is_static", "Device"."mac",
			"Network"."id" AS "Network.id",
				"Network"."label" AS "Network.label", "Network"."gateway", "Network"."netmask"
		FROM "Group-Device"
		JOIN "Device" ON "Group-Device"."Device.id" = "Device"."id"
		JOIN "Group" ON "Group-Device"."Group.id" = "Group"."id"
		JOIN "Network" ON "Device"."Network.id" = "Network"."id"
		WHERE "Network"."label" = $1
			AND "Group"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
		.bind(Network_label.clone())
		.bind(Group_label.clone())
		.fetch_all(pool).await?;

	let mut devices: Vec<Device> = vec![];
	for row in result
	{
		let Device_label: String = row.get("Device.label");
		let groups: Vec<Group> = SELECT_Groups_by_Device_label(pool, &Device_label).await?;
		devices.push(Device::new(groups, &row));
	}

	return Ok(devices);
}
