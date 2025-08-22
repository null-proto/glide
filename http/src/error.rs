// use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
  UnknownVersion,
  UnknownMethod,
  UriParse,
  HeaderParse,
  QueriParse,
  NoQueri,
}

// impl StdError for Error {
// }
