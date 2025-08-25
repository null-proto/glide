#![allow(unused)]

use crate::{error, header::uri::Uri};
use std::{collections::HashMap, ops::Deref, str::FromStr};

pub mod field;
pub mod uri;
pub mod status;

pub trait Parse<'a> {
  fn parse(s: &'a [u8]) -> Result<Self, error::Error>
  where
    Self: Sized;
}

#[derive(Default, Debug)]
pub struct HeaderMap<'a> {
  pub method: Method,
  pub(crate) uri: uri::Uri<'a>,
  pub version: Version,
  pub map: Option<HashMap<&'a str, &'a str>>,
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
  fn parse(s: &'a [u8]) -> Result<Self, error::Error>
  where
    Self: Sized,
  {
    let mut lines = str::from_utf8(s)
      .map_err(|_| error::Error::HeaderParse)?
      .split("\r\n");
    let mut status = lines.next().ok_or(error::Error::HeaderParse)?.split(' ');
    let method = status
      .next()
      .ok_or(error::Error::HeaderParse)?
      .parse::<Method>()?;
    let uri = Uri::parse(status.next().ok_or(error::Error::HeaderParse)?.as_bytes())?;
    let version = status
      .next()
      .ok_or(error::Error::HeaderParse)?
      .parse::<Version>()?;
    let map: Option<HashMap<&str, &str>> = lines
      .map(|kv| {
        if kv.trim().is_empty() {
          None
        } else {
          kv.split_once('=')
        }
      })
      .collect();

    Ok(Self {
      method,
      uri,
      version,
      map,
    })
  }
}

impl Deref for Method {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    match &self {
      Self::GET => &"GET",
      Self::HEAD => &"HEAD",
      Self::POST => &"POST",
      Self::PUT => &"PUT",
      Self::DELETE => &"DELETE",
      Self::CONNECT => &"CONNECT",
      Self::OPTIONS => &"OPTIONS",
      Self::TRACE => &"TRACE",
    }
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

impl Deref for Version {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    match self {
      Self::HTTP1 => &"HTTP/1.0",
      Self::HTTP11 => &"HTTP/1.1",
      Self::H2 => &"H2",
      Self::H3 => &"H3"
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
  use crate::header::Parse;

  use super::{HeaderMap, error};

  #[test]
  fn unit_test_header_map() {
    let req_string =
      "GET / HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n"
        .as_bytes();
    let header_map: Result<HeaderMap, error::Error> = HeaderMap::parse(req_string);
    println!("header::test::\n{:?}", header_map.unwrap());
  }
}
