use std::fmt::Display;
use std::{str::FromStr, sync::Arc};

use crate::header2::bytes::TryStr;
use crate::header2::uri::Uri;
use crate::{
  header::{Method, Version},
  header2::bytes::{ByteMap, Bytes},
};

pub mod bytes;
pub mod uri;

#[derive(Debug )]
pub struct Header {
  met: Method,
  uri: Uri,
  ver: Version,
  map: ByteMap,
}

impl Header {
  pub fn parse(s: Arc<[u8]>) -> Option<Self> {
    let mut p1 = 0;
    let mut p2 = 0;

    loop {
      p2 += 1;
      match s.get(p2) {
        Some(32) => break,
        Some(_) | None => {}
      }
    }

    let met: Method = match str::from_utf8(&s[p1..p2]) {
      Ok(m) => Method::from_str(m).ok(),
      Err(_) => None,
    }?;

    p1 = p2 + 1;

    let uri: Uri = Uri::parse(&s, p1)?;

    loop {
      p2 += 1;
      match s.get(p2) {
        Some(32) | None => break,
        Some(_) => {}
      }
    }

    p1 = p2 + 1;

    loop {
      p2 += 1;
      match s.get(p2) {
        Some(0x0D) | None => break,
        Some(_) => {}
      }
    }
    let ver: Version = match str::from_utf8(&s[p1..p2]) {
      Ok(v) => Version::from_str(v).ok(),
      Err(_) => None,
    }?;

    let mut map = ByteMap::default();

    'outer: loop {
      p2 += 2;
      p1 = p2;

      'key: loop {
        p2 += 1;
        match s.get(p2) {
          Some(0x3A) => break 'key, // `:`
          Some(0x0D) | None => break 'outer, // `\r`
          Some(_) => {}
        }
      }
      let k = Bytes::new(&s, p1, p2).into();
      println!(";; k = {}" , k);

      p1 = p2 + 1;
      'value: loop {
        p2 += 1;
        match s.get(p2) {
          Some(0x0D) => break 'value, // `\r`
          Some(_) => {}
          None => break 'outer,
        }
      }
      let v = Bytes::new(&s, p1, p2);
      println!(";; v = {}" , v);

      map.insert(k, v);
    }

    Some(Self { met, uri, ver, map })
  }
}

impl Header {
  pub fn method(&self) -> &Method {
    &self.met
  }

  pub fn version(&self) -> &Version {
    &self.ver
  }

  pub fn uri(&self) -> &Uri {
    &self.uri
  }

  pub fn get<'a>(&'a self , key : &'static str) -> Option<&'a str> {
    self.map.get(&bytes::ByteType::Str(key))?.try_str().map(|i| i.trim())
  }
}

impl Display for Header {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f , "Header : {:?} {} {:?}", self.met , self.uri , self.ver )
  }
}



#[cfg(test)]
mod header2 {
  use crate::header::field::{HOST, USER_AGENT};

use super::*;

  #[test]
  fn test_header() {
    let a : &[u8] = "GET / HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n".as_bytes();
    let b = Arc::from(a);
    let header = Header::parse(b).unwrap();
    println!(";; {}" , header);
    for (i,k) in &header.map {
      println!(";; [kv] {}  {}" , i , k);
    }
    assert!(header.get(USER_AGENT).unwrap().starts_with("curl/"));
  }

  #[test]
  fn test_header1() {
    let a : &[u8] = "GET /home.html HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n".as_bytes();
    let b = Arc::from(a);
    let header = Header::parse(b).unwrap();
    println!(";; {}" , header);
    assert!(header.get(USER_AGENT).unwrap().starts_with("curl/"));
  }

  #[test]
  fn test_header2() {
    let a : &[u8] = "GET /home.html?user=me HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n".as_bytes();
    let b = Arc::from(a);
    let header = Header::parse(b).unwrap();
    println!(";; {}" , header);
    assert!(header.get(USER_AGENT).unwrap().starts_with("curl/"));
  }

  #[test]
  fn test_header3() {
    let a : &[u8] = "GET /home.html?user=me&password=nah HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n".as_bytes();
    let b = Arc::from(a);
    let header = Header::parse(b).unwrap();
    println!(";; {}" , header);
    assert!(header.get(HOST).unwrap() == "[::]:8000");
  }


  #[test]
  fn test_header4() {
    let a : &[u8] = "GET /home.html?user=me&password=nah HTTP/1.1\r\nHost: [::]:8000\r\nUser-Agent: curl/8.15.0\r\nAccept: */*\r\n\r\n".as_bytes();
    let b = Arc::from(a);
    let header = Header::parse(b).unwrap();
    println!(";; {}" , header);
    assert_eq!(header.uri().get("password").unwrap() , "nah");
  }
}
