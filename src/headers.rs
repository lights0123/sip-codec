use http::header::{HeaderMap, HeaderValue};

pub mod allow;
pub mod content_length;
pub mod max_forwards;
pub mod user_agent;

pub trait ParseHeader: Sized {
	fn header_name() -> &'static [&'static str];
	fn decode<'a>(headers: impl IntoIterator<Item = &'a HeaderValue>) -> Option<Self>;
}

pub trait EncodeHeader: Into<String> {}

pub trait HeaderMapParse {
	fn typed_get<T: ParseHeader>(&self) -> Option<T>;
}

impl HeaderMapParse for HeaderMap {
	fn typed_get<T: ParseHeader>(&self) -> Option<T> {
		T::decode(
			T::header_name()
				.iter()
				.map(|name| self.get_all(*name))
				.flatten(),
		)
	}
}
