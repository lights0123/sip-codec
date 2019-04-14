use http::header::{HeaderName, HeaderValue};
use nom::is_space;

use super::is_newline;

fn is_colon(ch: u8) -> bool {
	ch == b':'
}

fn is_colon_or_space(ch: u8) -> bool {
	is_colon(ch) || is_space(ch)
}
named!(pub header<&[u8], (HeaderName, HeaderValue)>,
	do_parse!(
		name: map_res!(take_till1!(is_colon_or_space), HeaderName::from_bytes) >>
		opt!(is_a!(" \t")) >>
		char!(':') >>
		opt!(is_a!(" \t")) >>
		content: map_res!(take_till!(is_newline), HeaderValue::from_bytes) >>
        (name, content)
	)
);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_space() {
		let expected = (
			&b"\r\n"[..],
			(
				HeaderName::from_static("content-type"),
				HeaderValue::from_static("application/sdp"),
			),
		);
		assert_eq!(
			expected,
			header(b"Content-Type:application/sdp\r\n").unwrap()
		);
	}

	#[test]
	fn single_space() {
		let expected = (
			&b"\r\n"[..],
			(
				HeaderName::from_static("content-length"),
				HeaderValue::from_static("57"),
			),
		);
		assert_eq!(expected, header(b"Content-length: 57\r\n").unwrap());
	}

	#[test]
	fn double_space() {
		let expected = (
			&b"\r\n"[..],
			(
				HeaderName::from_static("cseq"),
				HeaderValue::from_static("314159 INVITE"),
			),
		);
		assert_eq!(expected, header(b"CSeq  :  314159 INVITE\r\n").unwrap());
	}
}
