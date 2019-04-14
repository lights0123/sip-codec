use std::fmt::{Display, Error, Formatter};

use http::header::HeaderValue;

use crate::headers::ParseHeader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaxForwards(pub u32);

impl ParseHeader for MaxForwards {
	fn header_name() -> &'static [&'static str] {
		&["max-forwards"]
	}

	fn decode<'a>(headers: impl IntoIterator<Item = &'a HeaderValue>) -> Option<Self> {
		headers
			.into_iter()
			.next()
			.and_then(|header| header.to_str().ok())
			.and_then(|value| value.parse().ok())
			.map(Self)
	}
}

impl Into<u32> for MaxForwards {
	fn into(self) -> u32 {
		self.0
	}
}

impl Display for MaxForwards {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", self.0)
	}
}
