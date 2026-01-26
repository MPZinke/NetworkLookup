
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use serde::Serialize;
use sqlx::FromRow;


// pub enum NetworkTypes
// {
// 	Asus("Asus"),
// 	NetGear("NetGear"),
// }


#[derive(FromRow, Serialize)]
pub struct Network
{
	pub id: i32,
	pub credentials: Option<String>,
	pub label: String,
	pub gateway: String,
	pub netmask: String,
	pub r#type: String,
}
