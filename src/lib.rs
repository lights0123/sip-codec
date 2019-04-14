#![forbid(unsafe_code)]
#![allow(clippy::write_with_newline)]
#[macro_use]
extern crate nom;

pub use codec::SIPCodec;
pub use request::Request;
pub use response::Response;

pub mod codec;
pub mod headers;
pub mod parser;
pub mod request;
pub mod response;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Version {
	pub major: u8,
	pub minor: u8,
}

impl Default for Version {
	fn default() -> Self {
		Self { major: 2, minor: 0 }
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Message {
	Request(Request),
	Response(Response),
}
