use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
  UnknownVersion,
  UnknownMethod,
  UriParse,
  HeaderParse,
}

// impl StdError for Error {
// }
