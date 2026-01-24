
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


use crate::db_tables::{device::Device, group::Group};


#[derive(Debug, Serialize)]
pub struct Service
{
	pub id: i32,
	pub auth_value: Option<String>,
	pub label: String,
	pub port: i16,
	pub device: Device
}


impl Service
{
	pub fn new(groups: Vec<Group>, row: &PgRow) -> Service
	{
		return Service{
			id: row.get("Service.id"),
			label: row.get("Service.label"),
			port: row.get("port"),
			auth_value: row.get("Service.auth_value"),
			device: Device::new(groups, row)
		};
	}
}
