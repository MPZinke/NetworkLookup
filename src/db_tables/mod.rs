
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


mod device;
mod network;
mod service;


pub use device::{Device as DBDevice, Group as Group, ToDBDeviceUpdateString as ToDBDeviceUpdateString};
pub use network::Network as Network;
pub use service::Service as Service;
