use std::sync::Arc;

use crate::header::HeaderMap;


pub struct Request<'a> {
  pub header: HeaderMap<'a>,
  pub body: Option<Arc<[u8]>>
}


impl<'a> Request<'a> {
  pub fn get_uri(&self) -> &'a str {
    self.header.uri.path
  }

  pub fn get_query(&self) -> Option<&std::collections::HashMap<&'a str, &'a str>> {
    self.header.uri.query.as_ref()
  }
}


#[cfg(test)]
mod request_unittest {
    use crate::header::{self, HeaderMap, Parse};


  #[test]
  fn test_request_w() {
    let sample = "POST / HTTP/1.1\r
Host: [::]:8000\r
User-Agent: curl/8.x.x\r
Accept: */*\r
Content-Length: 1\r
Content-Type: application/x-www-form-urlencoded\r
\r
w
";
    let head = HeaderMap::parse(sample.as_bytes());
    println!("{:?}", head.unwrap());
  }
}
