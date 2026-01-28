
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


use sqlx::{query_as, PgPool};


use crate::db_tables::Service;
use crate::lookup_error::LookupError;


pub async fn get_services(pool: &PgPool) -> Result<Vec<Service>, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Services".*,
			"Devices"."id" AS "Devices.id",
			"Devices"."label" AS "Devices.label",
			"Devices"."mac" AS "Devices.mac",
			"Devices"."static_ip_address" AS "Devices.static_ip_address",
			CASE WHEN "Devices"."band" IS NULL THEN NULL ELSE "Devices"."band"::TEXT END AS "Devices.band",
			"Devices"."Networks.id" AS "Devices.Networks.id",
			array_to_string(
				array(
					SELECT "Groups"."label"
					FROM "Groups-Devices"
					JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
					WHERE "Groups-Devices"."Devices.id" = "Devices"."id"
				),
				','
			) AS "Devices.groups"
		FROM "Services"
		JOIN "Devices" ON "Services"."Devices.id" = "Devices"."id";
	"#;
	return Ok(query_as::<_, Service>(query_str).fetch_all(pool).await?);
}


pub async fn get_service_by_id(pool: &PgPool, id: i32) -> Result<Service, LookupError>
{
	let query_str: &str = r#"
		SELECT
			"Services".*,
			"Devices"."id" AS "Devices.id",
			"Devices"."label" AS "Devices.label",
			"Devices"."mac" AS "Devices.mac",
			"Devices"."static_ip_address" AS "Devices.static_ip_address",
			CASE WHEN "Devices"."band" IS NULL THEN NULL ELSE "Devices"."band"::TEXT END AS "Devices.band",
			"Devices"."is_reservation" AS "Devices.is_reservation",
			"Devices"."Networks.id" AS "Devices.Networks.id",
			array_to_string(
				array(
					SELECT "Groups"."label"
					FROM "Groups-Devices"
					JOIN "Groups" ON "Groups-Devices"."Groups.id" = "Groups"."id"
					WHERE "Groups-Devices"."Devices.id" = "Devices"."id"
				),
				','
			) AS "Devices.groups"
		FROM "Services"
		JOIN "Devices" ON "Services"."Devices.id" = "Devices"."id"
		WHERE "Services"."id" = $1;
	"#;
	return Ok(query_as::<_, Service>(query_str).bind(id).fetch_one(pool).await?);
}
