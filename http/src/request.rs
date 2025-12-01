use std::{fmt::Display, sync::Arc};

use crate::header::{self, bytes::Bytes};
use crate::header::field;
use crate::error::Rp;
use tokio::io::{ AsyncRead , AsyncReadExt};

pub struct Request {
  pub header: header::Header,
  pub body: Option<Bytes>,
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

  pub async fn new<'a, T>(io: &'a mut T) -> Rp<Self>
  where
    T: AsyncRead + Unpin,
  {
    let mut data: Vec<u8> = Vec::new();
    let mut body: Vec<u8> = Vec::new();

    let mut buf = [0u8; 4096];

    'reader: loop {
      match io.read(&mut buf).await {
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
    let header = header::Header::parse(arc_data)?;
    if let Some(mut body_length) = header
      .get(field::CONTENT_LENGTH)
      .map(|i| i.parse::<usize>().unwrap())
    {
      loop {
        if body_length > body.len() {
          match io.read(&mut buf).await {
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
        } else {
          break;
        }
      }

      let f_body = Bytes::from(Arc::from(body), 0, body_length);
      println!("body {:?} ", f_body);

      Ok(Self {
        header: header,
        body: Some(f_body),
      })
    } else {
      Ok(Self { header: header, body: None })
    }
  }
}

impl Display for Request {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\nbody_size: {}\nbosy: {:?}",
      self.header,
      self.body.as_ref().map(|i| i.len() ).unwrap_or(0),
      self.body
    )
  }
}

#[cfg(test)]
mod unittest {
  use std::io::Cursor;

  use crate::request::Request;

  #[tokio::test]
  async fn request() {
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
    let res = Request::new(&mut sample).await;
    assert!(res.is_ok());
  }
}
