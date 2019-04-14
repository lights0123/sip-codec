use std::fmt::{Display, Error, Formatter};

use http::header::HeaderValue;

use crate::headers::ParseHeader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentLength(pub usize);

impl ParseHeader for ContentLength {
	fn header_name() -> &'static [&'static str] {
		&["content-length", "l"]
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

impl Into<usize> for ContentLength {
	fn into(self) -> usize {
		self.0
	}
}

impl Display for ContentLength {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", self.0)
	}
}
