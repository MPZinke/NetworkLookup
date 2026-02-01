
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


use serde::Serialize;
use sqlx::{FromRow, Row, postgres::PgRow};


use crate::db_tables::DBDevice;


#[derive(Debug, Serialize)]
pub struct Service
{
	pub id: i32,
	pub auth_value: Option<String>,
	pub domain: String,
	pub label: String,
	pub port: i16,
	pub device: DBDevice,
}


// FROM: https://stackoverflow.com/a/78618913
impl<'r> FromRow<'r, PgRow> for Service
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error>
    {
		Ok(
			Service {
				id: row.try_get::<i32, &str>("id")?,
				auth_value: row.try_get::<Option<String>, &str>("auth_value")?,
				domain: row.try_get::<String, &str>("domain")?,
				label: row.try_get::<String, &str>("label")?,
				port: row.try_get::<i16, &str>("port")?,
				device: DBDevice {
					band: row.try_get::<String, &str>("Devices.band")?,
					id: row.try_get::<i32, &str>("Devices.id")?,
					label: row.try_get::<String, &str>("Devices.label")?,
					mac: row.try_get::<String, &str>("Devices.mac")?,
					network_id: row.try_get::<i32, &str>("Devices.Networks.id")?,
					static_ip_address: row.try_get::<Option<String>, &str>("Devices.static_ip_address")?,
					groups: row.try_get::<String, &str>("Devices.groups")?.split(',').map(|s| s.to_string()).collect(),
				},
			}
		)
	}
}
