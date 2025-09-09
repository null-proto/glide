use std::sync::Arc;

use crate::header::{HeaderMap, Parse, field::CONTENT_LENGTH};

pub struct Request<'a> {
  pub header: HeaderMap<'a>,
  pub body: Option<Arc<[u8]>>,
}

impl<'a> Request<'a> {
  pub fn get_uri(&self) -> &'a str {
    self.header.uri.path
  }

  pub fn get_query(&self) -> Option<&std::collections::HashMap<&'a str, &'a str>> {
    self.header.uri.query.as_ref()
  }

  pub fn parse<'s>(data: &'s [u8]) -> Option<Request<'s>> {
    let head = HeaderMap::parse(data).ok()?;
    let body = match &head.map {
      Some(m) => {
        if let Some(size) = m.get(CONTENT_LENGTH) {
          if let Ok(size) = size.parse::<usize>() {
            let s = data.len() - size;
            Some(Arc::from(&data[s..data.len()]))
          } else {
            None
          }
        } else {
          None
        }
      }
      None => None,
    };

    Some(Request {
      header: head,
      body: body,
    })
  }
}

#[cfg(test)]
mod request_unittest {
  use crate::request::Request;

  #[test]
  fn test_request_w() {
    let sample = "POST /index.html/e?y=6&x=0 HTTP/1.1\r
Host: [::]:8000\r
User-Agent: curl/8.x.x\r
Accept: */*\r
Content-Length: 1\r
Content-Type: application/x-www-form-urlencoded\r
\r
w
";
    let req = Request::parse(sample.as_bytes()).unwrap();
    assert_eq!(req.get_uri(), "/index.html/e");
    assert_eq!(*req.get_query().unwrap().get("x").unwrap(), "0");
  }
}
