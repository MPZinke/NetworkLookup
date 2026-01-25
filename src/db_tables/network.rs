
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use serde::Serialize;
use sqlx::{Row, postgres::PgRow};


#[derive(Clone, Debug, Serialize)]
pub struct Network
{
	pub id: i32,
	pub label: String,
	pub gateway: String,
	pub netmask: String
}


impl Network
{
	pub fn new(row: &PgRow) -> Network
	{
		return Network{
			id: row.get("id"),
			label: row.get("label"),
			gateway: row.get("gateway"),
			netmask: row.get("netmask")
		};
	}
}


impl std::fmt::Display for Network
{
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
    {
		return write!(format, "{}", serde_json::to_string(self).unwrap());
    }
}
