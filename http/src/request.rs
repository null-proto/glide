use std::{io::Read, sync::Arc};

use crate::{
  header::field,
  header2::{self, Header, bytes::Bytes},
};

pub struct Request {
  pub header: header2::Header,
  pub body: Bytes,
}

impl Request {
  fn checker(a: &[u8]) -> Option<usize> {
    let mut j = 0;
    for i in a.windows(4) {
      j += 1;
      if i == [13, 10, 13, 10] {
        return Some(j + 4);
      };
    }
    None
  }

  fn read<'a, T>(io: &'a mut T) -> Option<Self>
  where
    T: Read,
  {
    let mut data: Vec<u8> = Vec::new();
    let mut body: Vec<u8> = Vec::new();

    let mut buf = [0u8; 4096];

    'reader: loop {
      match io.read(&mut buf) {
        Ok(0) => {
          break 'reader;
        }
        Ok(i) => {
          let loc = &buf[..i];
          if let Some(len) = Self::checker(loc) {
            data.extend_from_slice(&loc[..len]);
            body.extend_from_slice(&loc[len..]);
            break;
          } else {
            data.extend_from_slice(loc);
          }
        }
        Err(_e) => {}
      }
    }

    let arc_data = unsafe { Arc::from_raw(data.as_slice()) };

    let header = header2::Header::parse(arc_data)?;
    let mut body_length = header.get(field::CONTENT_LENGTH)?.parse::<usize>().ok()?;

    loop {
      match io.read(&mut buf) {
        Ok(0) => {}
        Ok(i) => {
          body_length -= i;
          body.extend_from_slice(&buf[..i]);
          if body.len() >= body_length {
            break;
          }
        }
        Err(_e) => {
          break;
        }
      }
    }

    Some(Self {
      header: header,
      body: Bytes::from(Arc::from(data), 0, body_length),
    })
  }

  pub fn read_from<'a, T>(io: &'a mut T) -> Option<Request>
  where
    T: Read,
  {
    Self::read(io)
  }
}

// #[cfg(test)]
// mod request_unittest {
//
//   #[test]
//   fn test_request_uri() {
//     let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
// Host: [::]:8000\r
// User-Agent: curl/8.x.x\r
// Accept: */*\r
// Content-Length: 1\r
// Content-Type: application/x-www-form-urlencoded\r
// \r
// w
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(req.get_uri(), "/index.html/e");
//   }
//
//   #[test]
//   fn test_request_query() {
//     let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
// Host: [::]:8000\r
// User-Agent: curl/8.x.x\r
// Accept: */*\r
// Content-Length: 1\r
// Content-Type: application/x-www-form-urlencoded\r
// \r
// w
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(*req.get_query().unwrap().get("x").unwrap(), "0");
//   }
//
//   #[test]
//   fn test_request_header_map() {
//     let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
// Host: [::]:8000\r
// User-Agent: curl/8.x.x\r
// Accept: */*\r
// Content-Length: 1\r
// Content-Type: application/x-www-form-urlencoded\r
// \r
// w
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(
//       *req.header.map.unwrap().get(field::CONTENT_LENGTH).unwrap(),
//       "1"
//     );
//   }
//
//   #[test]
//   fn test_request_g() {
//     let sample = "\
// GET /d/er.git/info/refs?service=git-upload-pack HTTP/1.1\r
// Host: [::1]:8000\r
// User-Agent: git/2.51.0\r
// Accept: */*\r
// Accept-Encoding: deflate, gzip, br, zstd\r
// Accept-Language: en-US, *;q=0.9\r
// Pragma: no-cacher\r
// Git-Protocol: version=2\r
// \r
// ";
//     let req = Request::parse(sample.as_bytes()).unwrap();
//     assert_eq!(req.get_uri(), "/d/er.git/info/refs");
//     assert_eq!(
//       *req.get_query().unwrap().get("service").unwrap(),
//       "git-upload-pack"
//     );
//   }
// }
