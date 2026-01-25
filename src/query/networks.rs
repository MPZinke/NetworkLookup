
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


use sqlx::{query, PgPool, postgres::PgRow};


use crate::db_tables::Network;
use crate::lookup_error::{new_not_found_error, LookupError};


pub async fn get_networks(pool: &PgPool) -> Result<Vec<Network>, LookupError>
{
	let query_str: &str = r#"
		SELECT *
		FROM "Networks";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut networks: Vec<Network> = vec![];
	for row in result
	{
		networks.push(Network::new(&row));
	}
	return Ok(networks);
}


pub async fn get_network_by_id(pool: &PgPool, id: i32) -> Result<Network, LookupError>
{
	let query_str: &str = r#"
		SELECT *
		FROM "Networks"
		WHERE "id" = $1;
	"#;
	let result: sqlx::Result<PgRow> = query(query_str).bind(id).fetch_one(pool).await;
	let row = match(result)
	{
		Ok(row) => row,
		Err(_) => return Err(new_not_found_error(format!("No results found for `Networks`.`id`: '{}'", id)))
	};

	return Ok(Network::new(&row));
}
