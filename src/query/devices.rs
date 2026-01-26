
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


use sqlx::{query_as, PgPool};


use crate::db_tables::Device;
use crate::lookup_error::LookupError;


pub async fn get_device_by_network_id_and_device_id(
	pool: &PgPool,
	network_id: i32,
	device_id: i32
) -> Result<Device, LookupError>
{
	let query_str: &str = r#"
		SELECT
			*,
			CASE WHEN "band" IS NULL THEN NULL ELSE CAST("band" AS TEXT) END,
			array_to_string(
				array(
					SELECT "Groups"."label"
					FROM "Groups-Devices"
					JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
					WHERE "Groups-Devices"."Devices.id" = "Devices"."id"
				),
				','
			) AS groups
		FROM "Devices"
		WHERE "Networks.id" = $1
			AND "id" = $2;
	"#;
	return Ok(query_as::<_, Device>(query_str).bind(network_id).bind(device_id).fetch_one(pool).await?);

	// let groups: Vec<Group> = row.get::<String, &str>("device_groups").split(',').map(|s| s.to_string()).collect();
	// return Ok(Device::new(&row, groups));
}


pub async fn get_devices_by_network_id(
	pool: &PgPool,
	network_id: i32
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			*,
			CASE WHEN "band" IS NULL THEN NULL ELSE CAST("band" AS TEXT) END,
			array_to_string(
				array(
					SELECT "Groups"."label"
					FROM "Groups-Devices"
					JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
					WHERE "Groups-Devices"."Devices.id" = "Devices"."id"
				),
				','
			) AS groups
		FROM "Devices"
		WHERE "Networks.id" = $1;
	"#;
	return Ok(query_as::<_, Device>(query_str).bind(network_id).fetch_all(pool).await?);
}



pub async fn get_devices_by_group_label(
	pool: &PgPool,
	group_label: String
) -> Result<Vec<Device>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Devices".*,
			CASE WHEN "band" IS NULL THEN NULL ELSE CAST("band" AS TEXT) END,
			array_to_string(
				array(
					SELECT "Groups"."label"
					FROM "Groups-Devices"
					JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
					WHERE "Groups-Devices"."Devices.id" = "Devices"."id"
				),
				','
			) AS groups
		FROM "Groups-Devices"
		JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
		JOIN "Devices" ON "Groups-Devices"."Devices.id" = "Devices"."id"
		WHERE "Groups"."label" = $1;
	"#;
	return Ok(query_as::<_, Device>(query_str).bind(&group_label).fetch_all(pool).await?);
}
