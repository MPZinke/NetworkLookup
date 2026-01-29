
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
use crate::response::ResponseError;


pub async fn get_groups(pool: &PgPool) -> Result<Vec<Group>, ResponseError>
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
