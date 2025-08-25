use std::sync::Arc;

use crate::header::HeaderMap;


pub struct Request<'a> {
  pub header: HeaderMap<'a>,
  pub body: Arc<[u8]>
}


impl<'a> Request<'a> {
  pub fn get_uri(&self) -> &'a str {
    self.header.uri.path
  }

  pub fn get_query(&self) -> Option<&std::collections::HashMap<&'a str, &'a str>> {
    self.header.uri.query.as_ref()
  }
}
