
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


use sqlx::{query, PgPool, postgres::PgRow};


use crate::db_tables::Group;
use crate::lookup_error::{NewNotFoundError, LookupError};


pub async fn SELECT_Groups(pool: &PgPool) -> Result<Vec<Group>, LookupError>
{
	let query_str: &str = r#"
		SELECT "id", "label"
		FROM "Groups";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut groups: Vec<Group> = vec![];
	for row in result
	{
		groups.push(Group::new(&row));
	}
	return Ok(groups);
}


pub async fn SELECT_Group_by_id(pool: &PgPool, id: i32) -> Result<Group, LookupError>
{
	let query_str: &str = r#"
		SELECT "id", "label"
		FROM "Groups"
		WHERE "id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(id).fetch_all(pool).await?;

	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Group`.`id`: '{}'", id)));
	}
	return Ok(Group::new(&result[0]));
}


pub async fn SELECT_Group_by_label(pool: &PgPool, label: &String) -> Result<Group, LookupError>
{
	let query_str: &str = r#"
		SELECT "id", "label"
		FROM "Groups"
		WHERE "label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(label.clone()).fetch_all(pool).await?;

	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Group`.`label`: '{}'", label)));
	}
	return Ok(Group::new(&result[0]));
}


// —————————————————————————————————————————————————— GROUP::BY Device —————————————————————————————————————————————————— //

pub async fn SELECT_Groups_by_Device_id(pool: &PgPool, Device_id: i32) -> Result<Vec<Group>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Groups"."id" AS "id", "Groups"."label" AS "label"
		FROM "Groups-Devices"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		WHERE "Groups-Devices"."Devices.id" = $1;
	"#;

	let result: Vec<PgRow> = query(query_str).bind(Device_id).fetch_all(pool).await?;

	let mut groups: Vec<Group> = vec![];
	for row in result
	{
		groups.push(Group::new(&row));
	}
	return Ok(groups);
}


pub async fn SELECT_Groups_by_Device_address(pool: &PgPool, Device_address: &String) -> Result<Vec<Group>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Groups"."id" AS "id", "Groups"."label"
		FROM "Groups-Devices"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		WHERE "Devices"."address" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(Device_address).fetch_all(pool).await?;

	let mut groups: Vec<Group> = vec![];
	for row in result
	{
		groups.push(Group::new(&row));
	}
	return Ok(groups);
}


pub async fn SELECT_Groups_by_Device_label(pool: &PgPool, Device_label: &String) -> Result<Vec<Group>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Groups"."id" AS "id", "Groups"."label" AS "label"
		FROM "Groups-Devices"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		WHERE "Devices"."label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(Device_label).fetch_all(pool).await?;

	let mut groups: Vec<Group> = vec![];
	for row in result
	{
		groups.push(Group::new(&row));
	}
	return Ok(groups);
}
