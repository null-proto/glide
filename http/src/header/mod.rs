#![allow(unused)]

use crate::error;
use std::{collections::HashMap, str::FromStr};

pub mod uri;
pub mod field;

pub trait Parse<'a> {
  fn parse(s:&'a [u8]) -> Result<Self,error::Error> where Self : Sized;
}

#[derive(Default, Debug)]
pub struct HeaderMap<'a> {
  method: Method,
  uri: uri::Uri<'a>,
  version: Version,
  map: HashMap<String, String>,
}

#[derive(Default, Debug)]
pub enum Method {
  #[default]
  GET,
  HEAD,
  POST,
  PUT,
  DELETE,
  CONNECT,
  OPTIONS,
  TRACE,
}

#[derive(Debug, Default)]
pub enum Version {
  HTTP1,
  #[default]
  HTTP11,
  H2,
  H3,
}

impl<'a> Parse<'a> for HeaderMap<'a> {
  fn parse(s:&'a [u8]) -> Result<Self,error::Error> where Self : Sized {
    let mut lines = str::from_utf8(s).map_err(|_|error::Error::HeaderParse)?.split("\r\n");
    let status = lines.next().ok_or(error::Error::HeaderParse)?.split(' ');
    todo!()
  }
}

impl FromStr for Method {
  type Err = error::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim() {
      "GET" => Ok(Self::GET),
      "HEAD" => Ok(Self::HEAD),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "DELETE" => Ok(Self::DELETE),
      "CONNECT" => Ok(Self::CONNECT),
      "OPTIONS" => Ok(Self::OPTIONS),
      "TRACE" => Ok(Self::TRACE),
      _ => Err(error::Error::UnknownMethod),
    }
  }
}

impl FromStr for Version {
  type Err = error::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim() {
      "HTTP/1.0" => Ok(Self::HTTP1),
      "HTTP/1.1" => Ok(Self::HTTP11),
      "H2" => Ok(Self::H2),
      "H3" => Ok(Self::H3),
      _ => Err(error::Error::UnknownVersion),
    }
  }
}

#[cfg(test)]
mod test {
  use super::{HeaderMap, error};

  #[test]
  fn unit_test_header_map() {
    let req_string =
      "GET / HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n";
    // let header_map: Result<HeaderMap, error::Error> = req_string.parse();
    // println!("header::test::\n{:?}", header_map.unwrap());
  }
}
