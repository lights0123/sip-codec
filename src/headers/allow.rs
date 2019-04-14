use std::fmt::{Display, Error, Formatter};

use http::header::HeaderValue;
use itertools::Itertools;

use crate::headers::ParseHeader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Allow(pub Vec<String>);

impl ParseHeader for Allow {
	fn header_name() -> &'static [&'static str] {
		&["allow"]
	}

	fn decode<'a>(headers: impl IntoIterator<Item = &'a HeaderValue>) -> Option<Self> {
		headers
			.into_iter()
			.next()
			.and_then(|header| header.to_str().ok())
			.map(|value| value.split(',').map(|method| method.trim().to_string()))
			.map(Iterator::collect)
			.map(Self)
	}
}

impl Display for Allow {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", self.0.iter().join(", "))
	}
}
