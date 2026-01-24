
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.21                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use crate::db_tables::network::Network;


pub enum DeviceAttributeSearch
{
	address(String),
	label(String),
	mac(String)
}


impl std::fmt::Debug for DeviceAttributeSearch
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match(self)
		{
			DeviceAttributeSearch::address(address) => write!(format, "{}", address),
			DeviceAttributeSearch::label(label) => write!(format, "{}", label),
			DeviceAttributeSearch::mac(mac) => write!(format, "{}", mac)
		}
	}
}


impl std::fmt::Display for DeviceAttributeSearch
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match(self)
		{
			DeviceAttributeSearch::address(_) => write!(format, "address"),
			DeviceAttributeSearch::label(_) => write!(format, "label"),
			DeviceAttributeSearch::mac(_) => write!(format, "mac")
		}
	}
}


impl DeviceAttributeSearch
{
	pub fn attribute(&self) -> &String
	{
		return match(self)
		{
			DeviceAttributeSearch::address(address) => &address,
			DeviceAttributeSearch::label(label) => &label,
			DeviceAttributeSearch::mac(mac) => &mac
		}
	}
}


// —————————————————————————————————————————————————— NETWORKSEACH —————————————————————————————————————————————————— //

pub enum NetworkSearch
{
	id(Network),
	label(Network)
}


impl std::fmt::Debug for NetworkSearch
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match(self)
		{
			NetworkSearch::id(network) => write!(format, "{}", network),
			NetworkSearch::label(network) => write!(format, "{}", network)
		}
	}
}


impl std::fmt::Display for NetworkSearch
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match(self)
		{
			NetworkSearch::id(_) => write!(format, "id"),
			NetworkSearch::label(_) => write!(format, "label")
		}
	}
}


impl NetworkSearch
{
	pub fn network(&self) -> &Network
	{
		return match(self)
		{
			NetworkSearch::id(network) => &network,
			NetworkSearch::label(network) => &network
		}
	}
}
