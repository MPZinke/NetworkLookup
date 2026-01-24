
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


use sqlx::{postgres::PgRow, Row};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Group
{
	pub id: i32,
	pub label: String
}


impl Group
{
	pub fn new(row: &PgRow) -> Group
	{
		return Group {
			id: row.get("id"),
			label: row.get("label")
		};
	}
}
