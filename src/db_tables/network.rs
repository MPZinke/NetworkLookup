
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


// #[macro_use] extern crate enum_str;


// enum_str! {
// 	NetworkType,
// 	(Asus, "Asus"),
// 	(NetGear, "NetGear"),
// }
pub enum NetworkType
{
	Asus,
	NetGear,
}


// impl NetworkType {
//     fn as_str(&self) -> &'static str
//     {
//         match self {
//             NetworkType::Asus => "Asus",
//             NetworkType::NetGear => "NetGear",
//         }
//     }
// }


#[derive(FromRow, Serialize)]
pub struct Network
{
	pub id: i32,
	pub credentials: Option<String>,
	pub label: String,
	pub gateway: String,
	pub netmask: String,
	pub r#type: Option<String>,
}
