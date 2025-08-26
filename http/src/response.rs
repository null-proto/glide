use std::{collections::HashMap, sync::Arc};

use crate::header::{self, Version, status::Status};

pub struct Response<'a> {
  version: Version,
  status: Status,
  map: HashMap<&'a str, String>,
  body: Option<&'a [u8]>,
}

impl<'a> Response<'a> {
  pub fn builder() -> Self {
    Self {
      version: Version::default(),
      status: Status(200),
      map: Default::default(),
      body: None,
    }
  }

  pub fn status(mut self, code: u16) -> Self {
    self.status = Status(code);
    self
  }

  pub fn insert(mut self, key: &'a str, value: String) -> Self {
    self.map.insert(key, value);
    self
  }

  pub fn add_body(mut self, c: &'a [u8]) -> Self {
    self
      .map
      .insert(header::field::CONTENT_LENGTH, c.len().to_string());
    self.body = Some(c);

    self
  }
}

pub trait Serialize {
  fn serialize(&self) -> Vec<u8>;
}

impl Serialize for Response<'_> {
  fn serialize(&self) -> Vec<u8> {
    let mut ser: Vec<u8> = [&*self.version, &self.status.0.to_string(), &*self.status]
      .join(" ")
      .into();
    ser.extend_from_slice("\r\n".as_bytes());
    for (k, v) in self.map.iter() {
      ser.extend_from_slice(
        &[
          k.as_bytes(),
          ": ".as_bytes(),
          v.as_bytes(),
          "\r\n".as_bytes(),
        ]
        .concat(),
      );
    }
    ser.extend_from_slice("\r\n".as_bytes());
    if let Some(body) = &self.body {
      ser.extend_from_slice(body);
    }
    ser
  }
}

#[cfg(test)]
mod unit_test_response {
  use super::*;
  use super::Serialize;
  use crate::header::field;

  #[test]
  fn serialize_test() {
    let res = Response::builder()
      .status(201)
      .insert(field::CONNECTION, "Close".to_owned())
      .add_body("text msg!".as_bytes());


    let msg =
      "HTTP/1.1 201 Ok\r\nConnection: Close\r\nContent-Length: 9\r\n\r\ntext msg!";

    let ser = res.serialize();
    println!("{:?}", ser);
  }
}
