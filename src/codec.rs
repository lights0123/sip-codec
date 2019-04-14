//! # A tokio-codec for sending and receiving SIP messages
use std::io;
use std::num::NonZeroUsize;

use bytes::BytesMut;
use nom::Err::{Error, Failure, Incomplete};
use tokio_codec::{Decoder, Encoder};

use crate::Message;
use crate::parser::parse_request;

/// # A [tokio-codec] for sending and receiving SIP messages
///
/// `SIPCodec` will automatically turn raw text into [`Message`]s, and vice versa. This can
/// be used to easily interface with tokio streams, as they have built-in support for codecs.
/// See the [Tokio docs] on framed streams for more.
///
/// Currently, it only supports decoding SIP requests.
///
/// [tokio-codec]: https://docs.rs/tokio-codec/
/// [Tokio docs]: https://tokio.rs/docs/going-deeper/frames/
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SIPCodec {
	max_size: Option<NonZeroUsize>,
}

impl Default for SIPCodec {
	fn default() -> Self {
		Self { max_size: NonZeroUsize::new(2_000_000) }
	}
}

impl Decoder for SIPCodec {
	type Item = Message;
	type Error = std::io::Error;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		match parse_request(&src[..]) {
			Err(Incomplete(_)) | Err(Error(_)) => Ok(None),
			Err(Failure(err)) => Err(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("{:?}", err),
			)),
			Ok((remaining, req)) => {
				let remaining_size = remaining.as_ptr() as usize - src.as_ptr() as usize;
				src.split_to(remaining_size);
				Ok(Some(Message::Request(req)))
			}
		}
	}
}

impl Encoder for SIPCodec {
	type Item = Message;
	type Error = std::io::Error;

	fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
		match item {
			Message::Response(res) => {
				res.write_buf(buf);
			}
			Message::Request(_req) => unimplemented!()
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use bytes::BytesMut;
	use http::header::HeaderValue;
	use http::HeaderMap;
	use tokio_codec::Decoder;

	use crate::{Message, Request, SIPCodec};

	#[test]
	fn test() {
		let mut bytes = BytesMut::new();
		let mut codec = SIPCodec::default();
		let initial = b"GET sip:user@server:port SIP/2.0\r\na:b\r\nContent-length: 7\r\n\r\nabcdef";
		bytes.extend_from_slice(initial);
		assert_eq!(None, codec.decode(&mut bytes).unwrap());
		bytes.extend_from_slice(b"g");
		let expected = {
			let mut headers = HeaderMap::new();
			headers.insert("a", HeaderValue::from_static("b"));
			headers.insert("content-length", HeaderValue::from_static("7"));
			Some(Message::Request(Request {
				method: "GET".into(),
				uri: "sip:user@server:port".into(),
				sip_version: Default::default(),
				headers,
				body: b"abcdefg".to_vec(),
			}))
		};
		assert_eq!(expected, codec.decode(&mut bytes).unwrap());
	}
}
