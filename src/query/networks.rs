
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


use crate::db_tables::Network;
use crate::lookup_error::LookupError;


pub async fn get_networks(pool: &PgPool) -> Result<Vec<Network>, LookupError>
{
	let query_str: &str = r#"
		SELECT *, CASE WHEN "type" IS NULL THEN NULL ELSE "type"::TEXT END
		FROM "Networks";
	"#;
	return Ok(query_as::<_, Network>(query_str).fetch_all(pool).await?);
}


pub async fn get_network_by_id(pool: &PgPool, id: i32) -> Result<Network, LookupError>
{
	let query_str: &str = r#"
		SELECT *, CASE WHEN "type" IS NULL THEN NULL ELSE "type"::TEXT END
		FROM "Networks"
		WHERE "id" = $1;
	"#;
	return Ok(query_as::<_, Network>(query_str).bind(id).fetch_one(pool).await?);
}
