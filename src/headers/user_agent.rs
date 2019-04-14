use std::fmt::{Display, Error, Formatter};

use http::header::HeaderValue;

use crate::headers::ParseHeader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserAgent(pub String);

impl ParseHeader for UserAgent {
	fn header_name() -> &'static [&'static str] {
		&["user-agent"]
	}

	fn decode<'a>(headers: impl IntoIterator<Item = &'a HeaderValue>) -> Option<Self> {
		headers
			.into_iter()
			.next()
			.and_then(|header| header.to_str().ok())
			.map(|value| value.into())
			.map(Self)
	}
}

impl Display for UserAgent {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", self.0)
	}
}
