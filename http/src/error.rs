// use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
  UnknownVersion,
  UnknownMethod,
  UriParse,
  HeaderParse,
  QueriParse,
  NoQueri,

  Byte2Str,
  Str2usize,
  ParseURI,
  ParseHeader,
  ParseBody,
  HttpVersion,
}

pub type Rp<T> = Result<T,Error>;

// impl StdError for Error {
// }
