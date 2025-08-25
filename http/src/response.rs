use std::{collections::HashMap, sync::Arc};

use crate::header::{Version, status::Status};

pub struct Response<'a> {
  version: Version,
  status: Status,
  map: HashMap<&'a str, &'a str>,
  body: Option<Arc<[u8]>>
}

impl<'a> Response<'a> {
  pub fn status(mut self, code: u16) -> Self {
    self.status = Status(code);
    self
  }

  pub fn insert(mut self, key: &'a str, value: &'a str) -> Self {
    self.map.insert(key, value);
    self
  }

  pub fn serialize(&self) -> &'a [u8] {
    let status_line: Vec<u8> = [&*self.version, &self.status.0.to_string(), &*self.status]
      .join(" ")
      .into();



    todo!()
  }
}
