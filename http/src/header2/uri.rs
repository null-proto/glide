use crate::header2::bytes::{ByteMap, Bytes, TryStr};
use std::sync::Arc;

///  `Uri( ref , start , end , path , option<query_string> , oprion<query_map> )`
#[derive(Debug, Clone)]
pub struct Uri<'a>(
  Arc<[u8]>,
  usize,
  usize,
  Bytes,
  Option<Bytes>,
  Option<ByteMap<'a>>,
);

impl<'a> Uri<'a> {
  pub fn parse(data: &Arc<[u8]>, start: usize) -> Option<Self> {
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

    // ?status=ok
    // 0123456789
    //

    if s {
      Some(Self(data.clone(), start, t, uri, None, None))
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

      Some(Self(data.clone(), start, t, uri, Some(query), Some(bmap)))
    }
  }
}

impl Uri<'_> {
  pub fn as_str<'a>(&'a self) -> Option<&'a str> {
    str::from_utf8(self.0.get(self.1..self.2)?).ok()
  }

  pub fn path(&self) -> Option<&str> {
    self.3.try_str()
  }

  pub fn query_str(&self) -> Option<&str> {
    self.4.as_ref()?.try_str()
  }

  pub fn get<'a>(&'a self, key: &'a str) -> Option<&'a str> {
    self.5.as_ref()?.get(&key.into()).unwrap().try_str()
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
    println!(";; uri {}", uri.3);
    let map = uri.5.clone().unwrap();

    for (k, v) in map.iter() {
      println!(";; k : {} \n;; v : {}", k, v);
    }

    assert_eq!(uri.get("status").unwrap(), "ok");
  }

  #[test]
  fn uri_parse_multiqueries() {
    let tags = Arc::from("GET /index.html/page?status=ok&k1=v1&k2=v2 HTTP/1.1 \r\n".as_bytes());
    let uri = Uri::parse(&tags, 4).unwrap();
    println!(";; uri {}", uri.3);
    let map = uri.5.clone().unwrap();

    for (k, v) in map.iter() {
      println!(";; k : {} \n;; v : {}", k, v);
    }

    assert_eq!(uri.get("status").unwrap(), "ok");
    assert_eq!(uri.get("k1").unwrap(), "v1");
    assert_eq!(uri.get("k2").unwrap(), "v2");
  }
}
