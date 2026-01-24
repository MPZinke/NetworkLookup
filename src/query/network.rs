
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


use sqlx::{query, PgPool, postgres::PgRow, Row};


use crate::db_tables::network::Network;
use crate::lookup_error::{NewNotFoundError, LookupError};


pub async fn SELECT_Networks(pool: &PgPool) -> Result<Vec<Network>, LookupError>
{
	let query_str: &str = r#"
	  SELECT "id", "label", "gateway", "netmask"
	  FROM "Networks";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut networks: Vec<Network> = vec![];
	for row in result
	{
		networks.push(Network::new(row.get("id"), row.get("label"), row.get("gateway"),
		  row.get("netmask")));
	}
	return Ok(networks);
}


pub async fn SELECT_Network_by_id(pool: &PgPool, id: i32) -> Result<Network, LookupError>
{
	let query_str: &str = r#"
	  SELECT "id", "label", "gateway", "netmask"
	  FROM "Networks"
	  WHERE "id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(id).fetch_all(pool).await?;
	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Network`.`id`: '{}'", id)));
	}

	return Ok(
		Network::new(
			result[0].get("id"),
			result[0].get("label"),
			result[0].get("gateway"),
			result[0].get("netmask")
		)
	);
}


pub async fn SELECT_Network_by_label(pool: &PgPool, label: &String) -> Result<Network, LookupError>
{
	let query_str: &str = r#"
	  SELECT "id", "label", "gateway", "netmask"
	  FROM "Networks"
	  WHERE "label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(label.clone()).fetch_all(pool).await?;
	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Network`.`label`: '{}'", label)));
	}

	return Ok(
		Network::new(
			result[0].get("id"),
			result[0].get("label"),
			result[0].get("gateway"),
			result[0].get("netmask")
		)
	);
}
