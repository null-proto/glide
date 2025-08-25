use std::{collections::HashMap, sync::Arc};

use crate::header::{self, status::Status, Version};

pub struct Response<'a> {
  version: Version,
  status: Status,
  map: HashMap<&'a str, String>,
  body: Option<&'a [u8]>
}

impl<'a> Response<'a> {
  pub fn status(mut self, code: u16) -> Self {
    self.status = Status(code);
    self
  }

  pub fn insert(mut self, key: &'a str, value: String) -> Self {
    self.map.insert(key, value);
    self
  }

  pub fn add_body(&mut self , c : &'a [u8]) {
    self.map.insert(header::field::CONTENT_LENGTH , c.len().to_string());
    self.body = Some(c);
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut ser: Vec<u8> = [&*self.version, &self.status.0.to_string(), &*self.status]
      .join(" ")
      .into();

    for (k,v) in self.map.iter() {
      ser.extend_from_slice(&[k.as_bytes() , ": ".as_bytes(), v.as_bytes() , "\r\n".as_bytes()].concat());
    }
    ser.extend_from_slice("\r\n".as_bytes());

    if let Some(body) = &self.body {
      ser.extend_from_slice(body);
    }
    ser
  }
}
