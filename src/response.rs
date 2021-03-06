use std::io::Write;

use bytes::BytesMut;
use http::HeaderMap;

use crate::Version;

fn code_str(code: u16) -> Option<&'static str> {
	match code {
		100 => Some("Trying"),
		180 => Some("Ringing"),
		181 => Some("Call is Being Forwarded"),
		182 => Some("Queued"),
		183 => Some("Session Progress"),
		199 => Some("Early Dialog Terminated"),
		200 => Some("OK"),
		202 => Some("Accepted"),
		204 => Some("No Notification"),
		300 => Some("Multiple Choices"),
		301 => Some("Moved Permanently"),
		302 => Some("Moved Temporarily"),
		305 => Some("Use Proxy"),
		380 => Some("Alternative Service"),
		400 => Some("Bad Request"),
		401 => Some("Unauthorized"),
		402 => Some("Payment Required"),
		403 => Some("Forbidden"),
		404 => Some("Not Found"),
		405 => Some("Method Not Allowed"),
		406 => Some("Not Acceptable"),
		407 => Some("Proxy Authentication Required"),
		408 => Some("Request Timeout"),
		409 => Some("Conflict"),
		410 => Some("Gone"),
		411 => Some("Length Required"),
		412 => Some("Conditional Request Failed"),
		413 => Some("Request Entity Too Large"),
		414 => Some("Request-URI Too Long"),
		415 => Some("Unsupported Media Type"),
		416 => Some("Unsupported URI Scheme"),
		417 => Some("Unknown Resource-Priority"),
		420 => Some("Bad Extension"),
		421 => Some("Extension Required"),
		422 => Some("Session Interval Too Small"),
		423 => Some("Interval Too Brief"),
		424 => Some("Bad Location Information"),
		428 => Some("Use Identity Header"),
		429 => Some("Provide Referrer Identity"),
		433 => Some("Anonymity Disallowed"),
		436 => Some("Bad Identity-Info"),
		437 => Some("Unsupported Certificate"),
		438 => Some("Invalid Identity Header"),
		439 => Some("First Hop Lacks Outbound Support"),
		440 => Some("Max-Breadth Exceeded"),
		469 => Some("Bad Info Package"),
		470 => Some("Consent Needed"),
		480 => Some("Temporarily Unavailable"),
		481 => Some("Call/Transaction Does Not Exist"),
		482 => Some("Loop Detected"),
		483 => Some("Too Many Hops"),
		484 => Some("Address Incomplete"),
		485 => Some("Ambiguous"),
		486 => Some("Busy Here"),
		487 => Some("Request Terminated"),
		488 => Some("Not Acceptable Here"),
		489 => Some("Bad Event"),
		491 => Some("Request Pending"),
		493 => Some("Undecipherable"),
		494 => Some("Security Agreement Required"),
		500 => Some("Server Internal Error"),
		501 => Some("Not Implemented"),
		502 => Some("Bad Gateway"),
		503 => Some("Service Unavailable"),
		504 => Some("Server Time-out"),
		505 => Some("Version Not Supported"),
		513 => Some("Message Too Large"),
		580 => Some("Precondition Failure"),
		600 => Some("Busy Everywhere"),
		603 => Some("Decline"),
		604 => Some("Does Not Exist Anywhere"),
		606 => Some("Not Acceptable"),
		607 => Some("Unwanted"),
		_ => None,
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
	pub version: Version,
	pub code: u16,
	pub headers: HeaderMap,
	pub body: Vec<u8>,
}

impl Response {
	pub fn new(code: u16) -> Self {
		Self {
			version: Default::default(),
			code,
			headers: HeaderMap::new(),
			body: vec![],
		}
	}

	pub fn write(&self, f: &mut impl Write) -> Result<(), std::io::Error> {
		let code: &str = code_str(self.code).unwrap_or("");
		write!(
			f,
			"SIP/{major}.{minor} {code}",
			major = self.version.major,
			minor = self.version.minor,
			code = self.code,
		)?;
		// If there's a string for the corresponding code, add a space and add it
		write!(
			f,
			"{space}{code_string}\r\n",
			space = if code.is_empty() { "" } else { " " },
			code_string = code,
		)?;
		for (key, value) in self.headers.iter() {
			if key == "content-length" {
				continue;
			}
			write!(f, "{}: {}\r\n", key, value.to_str().unwrap())?;
		}
		write!(f, "content-length: {}\r\n", self.body.len())?;
		write!(f, "\r\n")?;
		for letter in self.body.iter() {
			write!(f, "{}", *letter as char)?;
		}
		Ok(())
	}

	pub fn write_buf(&self, buf: &mut BytesMut) {
		let mut f = buf_writer::BufWriter::new(buf);
		self.write(&mut f).unwrap();
	}
}


/// This wraps a BytesMut and lets you use it with the `format!` macro. It will automatically
/// expand the buffer, unlike the preexisting `writer` function of BytesMut.
/// For example:
///
/// ```rust,ignore
/// let mut buf = bytes::BytesMut::new();
/// let mut f = BufWriter::new(&mut buf);
/// write!(f, "test");
/// assert_eq!(&b"test"[..], buf);
/// ```
mod buf_writer {
	use std::io;

	use bytes::BytesMut;
	pub struct BufWriter<'a> {
		buf: &'a mut BytesMut,
	}

	impl<'a> BufWriter<'a> {
		pub fn new(buf: &'a mut BytesMut) -> Self {
			Self { buf }
		}
	}

	impl<'a> io::Write for BufWriter<'a> {
		fn write(&mut self, src: &[u8]) -> io::Result<usize> {
			self.buf.extend_from_slice(src);
			Ok(src.len())
		}

		fn flush(&mut self) -> io::Result<()> {
			Ok(())
		}
	}
}

#[cfg(test)]
mod tests {
	use bytes::BytesMut;
	use http::header::HeaderValue;

	use super::Response;

	#[test]
	fn test() {
		let mut res = Response::new(200);

		res.headers.insert("a", HeaderValue::from_static("b"));
		res.headers
			.insert("content-length", HeaderValue::from_static("7"));
		res.body = b"abcdefg".to_vec();
		let mut buf = BytesMut::new();
		res.write_buf(&mut buf);
		let expected = b"SIP/2.0 200 OK\r\na: b\r\ncontent-length: 7\r\n\r\nabcdefg";
		assert_eq!(&expected[..], buf);
	}
}
