use std::net::IpAddr;

use http::header::HeaderMap;

use crate::Version;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Host {
	IpAddr(IpAddr),
	Hostname(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct URI {
	pub user: Option<String>,
	pub password: Option<String>,
	pub host: Host,
	pub port: Option<u16>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Request {
	pub method: String,
	pub uri: String,
	pub sip_version: Version,
	pub headers: HeaderMap,
	pub body: Vec<u8>,
}
