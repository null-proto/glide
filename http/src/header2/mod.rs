use std::{str::FromStr, sync::Arc};

use crate::header2::uri::Uri;
use crate::{
  header::{Method, Version},
  header2::bytes::{ByteMap, Bytes},
};

pub mod bytes;
pub mod uri;

struct Header {
  met: Method,
  uri: Uri,
  ver: Version,
  map: ByteMap,
}

impl Header {
  fn parse(s: Arc<[u8]>) -> Option<Self> {
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
          Some(0x3A) => break 'key,
          Some(0x03) | None => break 'outer, // `:`
          Some(_) => {}
        }
      }
      let k = Bytes::new(&s, p1, p2).into();

      p1 = p2 + 1;
      'value: loop {
        p2 += 1;
        match s.get(p2) {
          Some(0x03) => break 'value, // `\r`
          Some(_) => {}
          None => break 'outer,
        }
      }
      let v = Bytes::new(&s, p1, p2);

      map.insert(k, v);
    }

    Some(Self { met, uri, ver, map })
  }
}
