
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


use serde::Serialize;
use sqlx::{FromRow, Row, postgres::PgRow};


pub type Group = String;


#[derive(Debug, Serialize)]
pub struct Device
{
	pub band: Option<String>,
	pub id: i32,
	pub is_reservation: bool,
	pub label: String,
	pub mac: String,
	pub network_id: i32,
	pub static_ip_address: Option<String>,
	pub groups: Vec<Group>,
}


// FROM: https://stackoverflow.com/a/78618913
impl<'r> FromRow<'r, PgRow> for Device
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error>
    {
		Ok(
			Device {
				band: row.try_get::<Option<String>, &str>("band")?,
				id: row.try_get::<i32, &str>("id")?,
				is_reservation: row.try_get::<bool, &str>("is_reservation")?,
				label: row.try_get::<String, &str>("label")?,
				mac: row.try_get::<String, &str>("mac")?,
				network_id: row.try_get::<i32, &str>("Networks.id")?,
				static_ip_address: row.try_get::<Option<String>, &str>("static_ip_address")?,
				groups: row.try_get::<String, &str>("groups")?.split(',').map(|s| s.to_string()).collect(),
			}
		)
	}
}
