
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.09.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use sqlx::{postgres::PgRow, Row};
use serde::Serialize;


use crate::db_tables::{Device};


#[derive(Debug, Serialize)]
pub struct Service
{
	pub id: i32,
	pub auth_value: Option<String>,
	pub domain: String,
	pub label: String,
	pub port: i16,
	pub device: Device
}


impl Service
{
	pub fn new(device: Device, row: &PgRow) -> Service
	{
		return Service{
			id: row.get("id"),
			auth_value: row.get("auth_value"),
			domain: row.get("domain"),
			label: row.get("label"),
			port: row.get("port"),
			device: device
		};
	}
}
