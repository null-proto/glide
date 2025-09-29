use std::net::TcpListener;

use http::request::{self, Request};
use tracing::{debug, info, trace, warn};


pub mod git;

pub fn serve(listener: TcpListener) {

  while let Ok((mut stream , peer)) = listener.accept() {
    info!("connection form : {}", peer);

    match Request::new(&mut stream) {

      Ok(req) => {
        trace!("{}", req);

        let uri = req.header.uri();
        debug!("{}" , uri.path().unwrap());
      }

      Err(e) => {
        warn!("request cancelled {}, cuz {:?}" , peer , e);
      }

    }
  }
}
