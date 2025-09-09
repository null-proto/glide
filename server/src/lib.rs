use std::net::TcpListener;

use http::{
  header::{self, HeaderMap, Parse},
  request::Request,
};
use tracing::{info, trace};

use crate::{
  preludes::{read_body, read_request_bytes},
  routes::Router,
};

pub mod git;
pub mod preludes;
pub mod routes;

pub fn serve(listener: TcpListener) {
  let mut router = Router {};
  info!("server started");

  while let Ok((mut stream, peer)) = listener.accept() {
    trace!("connection from {:?}", peer);
    let req_b = read_request_bytes(&mut stream);

    if let Ok(header) = HeaderMap::parse(&req_b) {
      trace!("header : {:?}", header);
      let mut req = Request {
        header: header,
        body: None,
      };

      if let Some(map) = &req.header.map {
        if let Some(len) = map.get(header::field::CONTENT_LENGTH) {
          trace!("body with length {:?}", len);
          req.body = Some(read_body(&mut stream, len.parse::<usize>().unwrap()));
        }
      }

      router.route(stream, req);
    } else {
      trace!("invalid header http/1.1");
      trace!("colosing connection peer: {}", peer);
      let _ = stream.shutdown(std::net::Shutdown::Both);
    }
  }
}
