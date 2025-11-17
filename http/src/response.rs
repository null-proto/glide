use std::sync::Arc;

use crate::header::field::SERVER;

pub struct Response(Arc<[u8]>);

impl Response {
  pub fn get(self) -> Arc<[u8]> {
    self.0
  }

  pub fn build<'a>() -> ResponseBuilder<'a> {
    ResponseBuilder {
      status: 200,
      status_text: "ok",
      map: vec![(SERVER, "glide")],
      body: None,
    }
  }
}

pub struct ResponseBuilder<'a> {
  status: u16,
  status_text: &'a str,
  map: Vec<(&'a str, &'a str)>,
  body: Option<&'a [u8]>,
}

impl<'a> ResponseBuilder<'a> {
  pub fn status(mut self, code: u16) -> Self {
    self.status = code;
    self
  }

  pub fn status_text(mut self, msg: &'a str) -> Self {
    self.status_text = msg;
    self
  }

  pub fn header(mut self, k: &'a str, v: &'a str) -> Self {
    self.map.push((k, v));
    self
  }

  pub fn body(mut self, body: &'a [u8]) -> Self {
    self.body = Some(body);
    self
  }
}

impl<'a> ResponseBuilder<'a> {
  pub fn finish(self) -> Response {
    let mut seri: Vec<u8> = Vec::with_capacity(4096);

    seri.extend_from_slice("HTTP/1.1".as_bytes());
    seri.push(0x20);
    seri.extend_from_slice(self.status.to_string().as_bytes());
    seri.push(0x20);
    seri.extend_from_slice(self.status_text.as_bytes());
    seri.push(0x0D);
    seri.push(0x0A);
    for (k, v) in self.map {
      seri.extend_from_slice(k.as_bytes());
      seri.push(0x3A);
      seri.push(0x20);
      seri.extend_from_slice(v.as_bytes());
      seri.push(0x0D);
      seri.push(0x0A);
    }
    seri.push(0x0D);
    seri.push(0x0A);
    if let Some(body) = self.body {
      seri.extend_from_slice(body);
    }

    Response(Arc::from(seri))
  }

  pub fn attach_raw<T>(self,bytes : T) -> Response where T: Into<&'a [u8]> {
    let mut seri: Vec<u8> = Vec::with_capacity(4096);
    seri.extend_from_slice("HTTP/1.1".as_bytes());
    seri.push(0x20);
    seri.extend_from_slice(self.status.to_string().as_bytes());
    seri.push(0x20);
    seri.extend_from_slice(self.status_text.as_bytes());
    seri.push(0x0D);
    seri.push(0x0A);
    for (k, v) in self.map {
      seri.extend_from_slice(k.as_bytes());
      seri.push(0x3A);
      seri.push(0x20);
      seri.extend_from_slice(v.as_bytes());
      seri.push(0x0D);
      seri.push(0x0A);
    }
    seri.extend_from_slice(bytes.into());
    Response(Arc::from(seri))
  }
}

#[cfg(test)]
mod unittest {
  #[test]
  fn response() {}
}
