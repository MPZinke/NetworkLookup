
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


use sqlx::{query, PgPool, Row, postgres::PgRow};


use crate::db_tables::Group;
use crate::lookup_error::{LookupError, new_not_found_error};


pub async fn get_groups(pool: &PgPool) -> Result<Vec<Group>, LookupError>
{
	let query_str: &str = r#"
		SELECT *
		FROM "Groups";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut groups: Vec<Group> = vec![];
	for row in result
	{
		groups.push(row.get("label"));
	}
	return Ok(groups);
}


pub async fn get_group_by_id(pool: &PgPool, id: i32) -> Result<Group, LookupError>
{
	let query_str: &str = r#"
		SELECT *
		FROM "Groups"
		WHERE "id" = $1;
	"#;
	let result: sqlx::Result<PgRow> = query(query_str).bind(id).fetch_one(pool).await;
	let row = match(result)
	{
		Ok(row) => row,
		Err(_) => return Err(new_not_found_error(format!("No results found for `Group`.`id`: '{}'", id)))
	};

	return Ok(row.get("label"));
}


pub async fn get_groups_by_device_id(pool: &PgPool, device_id: i32) -> Result<Vec<Group>, LookupError>
{
	let query_str: &str = r#"
		SELECT "Groups".*
		FROM "Groups-Devices"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		WHERE "Groups-Devices"."Devices.id" = $1;
	"#;

	let result: Vec<PgRow> = query(query_str).bind(device_id).fetch_all(pool).await?;

	let mut groups: Vec<Group> = vec![];
	for row in result
	{
		groups.push(row.get("label"));
	}
	return Ok(groups);
}
