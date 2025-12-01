use crate::header2::bytes::{ByteMap, Bytes, TryStr};
use std::{fmt::Display, sync::Arc};
use crate::error::Rp;

#[derive(Debug, Clone)]
pub struct Uri(
  Bytes,
  Option<Bytes>,
  Option<ByteMap>,
);

impl Uri {
  pub fn parse(data: &Arc<[u8]>, start: usize) -> Rp<Self> {
    let mut t = start - 1;
    let mut s = false;

    'uri_reader: for i in &data[start..] {
      t += 1;
      match i {
        63 => {
          break 'uri_reader;
        }
        32 => {
          s = true;
          break 'uri_reader;
        }
        _ => {}
      }
    }

    let uri = Bytes::new(data, start, t);

    if s {
      Ok(Self(uri, None, None))
    } else {
      let mut p1 = t + 1;
      let mut p2 = 0usize;
      let mut bmap = ByteMap::default();
      let qstart = t;

      'query_reader: for i in &data[t + 1..] {
        t += 1;
        match i {
          32 => {
            let k = Bytes::new(&data, p1, p2);
            let v = Bytes::new(&data, p2 + 1, t);
            bmap.insert(k.into(), v);
            break 'query_reader;
          }

          61 => {
            // =
            p2 = t;
          }

          38 => {
            // &
            let k = Bytes::new(&data, p1, p2);
            let v = Bytes::new(&data, p2 + 1, t);
            bmap.insert(k.into(), v);
            p1 = t + 1;
          }
          _ => {}
        }
      }
      let query = Bytes::new(&data, qstart, t);

      Ok(Self(uri, Some(query), Some(bmap)))
    }
  }
}

impl Uri {

  pub fn path(&self) -> Option<&str> {
    self.0.try_str()
  }

  pub fn query_str(&self) -> Option<&str> {
    self.1.as_ref()?.try_str()
  }

  pub fn get<'a>(&'a self, key: &'static str) -> Option<&'a str> {
    self.2.as_ref()?.get(&key.into()).unwrap().try_str()
  }
}

impl Display for Uri {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f , "{}" , match &self.1 { Some(i) => format!("{}?{}",self.0,i) ,None=> self.0.to_string() })
  }
}

#[cfg(test)]
mod header2_uri_unit_test {
  use std::sync::Arc;

  use crate::header2::uri::Uri;

  #[test]
  fn uri_parse_simple() {
    let tags = Arc::from("GET / HTTP/1.1 \r\n".as_bytes());
    let uri = Uri::parse(&tags, 4).unwrap();
    assert_eq!(uri.path().unwrap(), "/");
  }

  #[test]
  fn uri_parse_normal() {
    let tags = Arc::from("GET /index.html/page?status=ok HTTP/1.1 \r\n".as_bytes());
    let uri = Uri::parse(&tags, 4).unwrap();
    assert_eq!(uri.path().unwrap(), "/index.html/page");
  }

  #[test]
  fn uri_parse_queries() {
    let tags = Arc::from("GET /index.html/page?status=ok HTTP/1.1 \r\n".as_bytes());
    let uri = Uri::parse(&tags, 4).unwrap();
    let map = uri.2.clone().unwrap();
    assert_eq!(uri.get("status").unwrap(), "ok");
  }

  #[test]
  fn uri_parse_multiqueries() {
    let tags = Arc::from("GET /index.html/page?status=ok&k1=v1&k2=v2 HTTP/1.1 \r\n".as_bytes());
    let uri = Uri::parse(&tags, 4).unwrap();
    let map = uri.2.clone().unwrap();
    assert_eq!(uri.get("status").unwrap(), "ok");
    assert_eq!(uri.get("k1").unwrap(), "v1");
    assert_eq!(uri.get("k2").unwrap(), "v2");
  }
}
