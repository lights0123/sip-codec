use std::str;
use std::string::FromUtf8Error;

use failure::Error;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use nom::{is_digit, is_space, rest, IResult};

use crate::headers::content_length::ContentLength;
use crate::headers::HeaderMapParse;
use crate::request::Request;
use crate::Version;

mod header;

fn is_newline(ch: u8) -> bool {
	ch == b'\r' || ch == b'\n'
}

fn slice_to_string(slice: &[u8]) -> Result<String, FromUtf8Error> {
	String::from_utf8(Vec::from(slice))
}

fn parse_u8(slice: &[u8]) -> Result<u8, Error> {
	Ok(str::from_utf8(slice)?.parse()?)
}

enum BodyLength {
	All,
	Length(usize),
}

fn get_body_length(map: &HeaderMap) -> BodyLength {
	map.typed_get::<ContentLength>()
		.map(Into::into)
		.map(BodyLength::Length)
		.unwrap_or(BodyLength::All)
}

fn body(input: &[u8], length: BodyLength) -> IResult<&[u8], Vec<u8>> {
	match length {
		BodyLength::All => rest(input),
		BodyLength::Length(len) => take!(input, len),
	}
	.map(|(rem, res)| (rem, res.into()))
}

named!(lf, alt!(tag!("\r\n") | tag!("\r") | tag!("\n")));

named!(
	headers<HeaderMap>,
	map!(
		many0!(complete!(terminated!(header::header, lf))),
		make_header_map
	)
);

named!(
	version<Version>,
	do_parse!(
		tag!("SIP/")
			>> major: map_res!(take_while1!(is_digit), parse_u8)
			>> char!('.')
			>> minor: map_res!(take_while1!(is_digit), parse_u8)
			>> (Version { major, minor })
	)
);

named!(
	string_until_space<String>,
	map_res!(take_till1!(is_space), slice_to_string)
);

named!(pub parse_request<&[u8], Request>,
	do_parse!(
		many0!(lf) >>
 		method: string_until_space >>
 		take_while1!(is_space) >>
 		uri: string_until_space >>
 		take_while1!(is_space) >>
 		sip_version: version >>
 		lf >>
 		headers: headers >>
 		lf >>
 		body: call!(body, get_body_length(&headers)) >>
        (RequestBuilder {method, uri, sip_version, headers, body: body}.into())
	)
);

struct RequestBuilder {
	pub method: String,
	pub uri: String,
	pub sip_version: Version,
	pub headers: HeaderMap,
	pub body: Vec<u8>,
}

impl Into<Request> for RequestBuilder {
	fn into(self) -> Request {
		Request {
			method: self.method,
			uri: self.uri,
			sip_version: self.sip_version,
			headers: self.headers,
			body: self.body,
		}
	}
}

fn make_header_map(headers: Vec<(HeaderName, HeaderValue)>) -> HeaderMap {
	let mut map = HeaderMap::new();
	for (name, value) in headers {
		map.insert(name, value);
	}
	map
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_space() {
		let mut headers = HeaderMap::new();
		headers.insert("a", HeaderValue::from_static("b"));
		headers.insert("content-length", HeaderValue::from_static("7"));

		let expected = Ok((
			&[][..],
			Request {
				method: "GET".into(),
				uri: "sip:user@server:port".into(),
				sip_version: Default::default(),
				headers,
				body: b"abcdefg".to_vec(),
			},
		));
		assert_eq!(
			expected,
			parse_request(
				b"GET sip:user@server:port SIP/2.0\r\na:b\r\nContent-length: 7\r\n\r\nabcdefg"
			)
		);
	}

	#[test]
	fn shorthand() {
		let mut headers = HeaderMap::new();
		headers.insert("a", HeaderValue::from_static("b"));
		headers.insert("l", HeaderValue::from_static("7"));

		let expected = Ok((
			&b"h"[..],
			Request {
				method: "GET".into(),
				uri: "sip:user@server:port".into(),
				sip_version: Default::default(),
				headers,
				body: b"abcdefg".to_vec(),
			},
		));
		assert_eq!(
			expected,
			parse_request(b"GET sip:user@server:port SIP/2.0\r\na:b\r\nl: 7\r\n\r\nabcdefgh")
		);
	}
}
