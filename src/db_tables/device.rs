
/***********************************************************************************************************************
**
* created by: MPZinke*
* on 2022.05.05*
**
* DESCRIPTION: TEMPLATE*
* BUGS:*
* FUTURE:*
**
***********************************************************************************************************************/


use sqlx::{Row, postgres::PgRow};
use serde::Serialize;


use crate::db_tables::Network;


pub type Group = String;


#[derive(Debug, Serialize)]
pub struct Device
{
	pub id: i32,
	pub address: Option<String>,
	pub label: String,
	pub is_reservation: bool,
	pub is_static: bool,
	pub mac: Option<String>,
	pub groups: Vec<Group>,
	pub network: Network
}


impl Device
{
	pub fn new(row: &PgRow, groups: Vec<Group>, network: Network) -> Device
	{
		return Device {
			id: row.get("id"),
			address: row.get("address"),
			label: row.get("label"),
			is_reservation: row.get("is_reservation"),
			is_static: row.get("is_static"),
			mac: row.get("mac"),
			groups: groups,
			network: network,
		};
	}
}
