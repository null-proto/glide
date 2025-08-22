use std::sync::Arc;

use crate::header::HeaderMap;


pub struct Request<'a> {
  header: HeaderMap<'a>,
  body: Arc<[u8]>
}
