use std::{net::TcpStream, sync::Arc};
use std::io::Read;

use http::request::Request;
use http::response::Response;
use tracing::trace;


pub fn read_request_bytes(stream: &mut TcpStream) -> Arc<[u8]> {
  let mut buf = [0; 512];
  let mut req = vec![];

  trace!("reached header reader");

  'outer: loop {
    match stream.read(&mut buf) {
      Ok(0) => {
        break 'outer;
      }
      Ok(n) => {
        req.extend_from_slice(&buf[..n]);
        if buf[n - 4..n].eq(&[0x0D, 0x0A, 0x0D, 0x0A]) {
          break;
        }
      }
      Err(_) => {
        break;
      }
    }
  }

  trace!("head map read ok");

  Arc::from(req)
}

pub fn read_body(stream: &mut TcpStream , len : usize ) -> Arc<[u8]> {
  let mut buf = [0; 512];
  let mut req = vec![];

  trace!("reached body reader");

  'outer: loop {
    match stream.read(&mut buf) {
      Ok(0) => {
        break 'outer;
      }
      Ok(n) => {
        req.extend_from_slice(&buf[..n]);
        if req.len()>len {
          break;
        }
      }
      Err(_) => {
        break;
      }
    }
  }
  trace!("body read completed");

  Arc::from(req)
}
