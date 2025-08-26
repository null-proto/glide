use std::net::TcpListener;

use http::header::{self, HeaderMap, Parse};
use tracing::{info, trace};

use crate::{preludes::{read_body, read_request_bytes}, routes::Router};

pub mod git;
pub mod preludes;
pub mod routes;

pub fn serve(listener: TcpListener) {
  let mut router = Router{};
  info!("server started");

  while let Ok(mut stream) = listener.accept() {
    trace!("connection from {:?}", stream.1);
    let req_b = read_request_bytes(&mut stream.0);
    let header_map = HeaderMap::parse(&req_b).map_err(|e| {
      trace!("header parse error {:?}", e);
    }).unwrap();

    let mut req = http::request::Request { header: header_map, body: None };

    if let Some(map) = &req.header.map {
      if let Some(len) = map.get(header::field::CONTENT_LENGTH) {
        trace!("body with length {:?}", len);
        req.body = Some(read_body(&mut stream.0, len.parse::<usize>().unwrap() ));
      }
    }

    router.route(stream.0, req);
  }
}
