
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
		groups.push(Group::new(&row));
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
	let result: Vec<PgRow> = query(query_str).bind(id).fetch_all(pool).await?;

	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Group`.`id`: '{}'", id)));
	}
	return Ok(Group::new(&result[0]));
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
		groups.push(Group::new(&row));
	}
	return Ok(groups);
}
