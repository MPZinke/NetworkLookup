
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
	pub id: Option<i32>,
	pub address: Option<String>,
	pub label: String,
	pub is_reservation: Option<bool>,
	pub is_static: Option<bool>,
	pub is_reachable: Option<bool>,
	pub mac: Option<String>,
	pub groups: Vec<Group>,
	pub network: Network
}


impl Device
{
	pub fn new(row: &PgRow, groups: Vec<Group>, network: Network) -> Device
	{
		return Device {
			id: Some(row.get("id")),
			address: row.get("address"),
			label: row.get("label"),
			is_reservation: Some(row.get("is_reservation")),
			is_static: Some(row.get("is_static")),
			is_reachable: None,
			mac: row.get("mac"),
			groups: groups,
			network: network,
		};
	}
}
