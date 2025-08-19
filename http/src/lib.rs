use std::{collections::HashMap, str::FromStr};

pub mod error;
pub mod header;
pub mod uri;

#[derive(Default,Debug)]
pub struct HeaderMap {
  method: HTTPMethods,
  uri: uri::Uri,
  version: HTTPVersion,
  map: HashMap<String, String>,
}

#[derive(Default,Debug)]
pub enum HTTPMethods {
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

#[derive(Debug,Default)]
pub enum HTTPVersion {
  HTTP1,
  #[default]
  HTTP11,
  H2,
  H3,
}

impl FromStr for HTTPMethods {
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

impl FromStr for HTTPVersion {
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

impl FromStr for HeaderMap {
  type Err = error::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut itermap = s.split("\r\n");
    let (method, uri, version) = {
      let mut a = itermap.next().ok_or(error::Error::HeaderParse)?.split(" ");
      let method = a
        .next()
        .ok_or(error::Error::HeaderParse)?
        .parse::<HTTPMethods>()?;
      let uri = a
        .next()
        .ok_or(error::Error::UriParse)?
        .parse::<uri::Uri>()?;
      let version = a
        .next()
        .ok_or(error::Error::UriParse)?
        .parse::<HTTPVersion>()?;
      (method, uri, version)
    };
    let header_map: HashMap<String, String> = itermap
      .filter_map(|i: &str| {
        let a = i.split_once(':');
        a.map(|i| (i.0.trim().to_owned(), i.1.trim().to_owned()))
      })
      .collect();

    Ok(Self {
      method,
      uri,
      version,
      map: header_map,
    })
  }
}



#[cfg(test)]
mod test {
    use crate::{error, HeaderMap};

  #[test]
  fn unit_test_header_map() {
    let req_string = "GET / HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n";
    let header_map : Result<HeaderMap, error::Error> = req_string.parse();
    println!("header::test::\n{:?}",header_map.unwrap());
  }
}

