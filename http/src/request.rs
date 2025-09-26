use std::{io::Read, sync::Arc};

use crate::{
  header::field,
  header2::{self, bytes::Bytes},
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
        return Some(j + 3);
      };
    }
    None
  }

  pub fn new<'a, T>(io: &'a mut T) -> Option<Self>
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

    let arc_data = Arc::from(data);
    let header = header2::Header::parse(arc_data)?;
    let mut body_length = header.get(field::CONTENT_LENGTH)?.parse::<usize>().ok()?;

    loop {
      if body_length > body.len() {
        match io.read(&mut buf) {
          Ok(0) => {
            break;
          }

          Ok(i) => {
            body_length -= i;
            body.extend_from_slice(&buf[..i]);
          }
          Err(_e) => {
            break;
          }
        }
      }
      else {
        break;
      }
    }

    let f_body = Bytes::from(Arc::from(body) , 0 , body_length);
    println!("body {:?} ", f_body);

    Some(Self {
      header: header,
      body: f_body
    })
  }
}

#[cfg(test)]
mod unittest {
  use std::io::Cursor;

  use crate::request::Request;

  #[test]
  fn request() {
    let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
Host: [::]:8000\r
User-Agent: curl/8.x.x\r
Accept: */*\r
Content-Length: 1\r
Content-Type: application/x-www-form-urlencoded\r
\r
w
";
    let mut sample = Cursor::new(sample.as_bytes());
    let res = Request::new(&mut sample);
    assert!(res.is_some());
  }
}
