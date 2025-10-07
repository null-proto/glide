use std::{io::Write, net::TcpListener};

use http::{request::{self, Request}, response::{Response, ResponseBuilder}};
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

        match uri {
          _ => {
            let res = Response::build()
              .status(404)
              .status_text("Not Found")
              .finish();
            _ = stream.write_all(&res.get());
          }
        }
      }

      Err(e) => {
        warn!("request cancelled {}, cuz {:?}" , peer , e);
      }

    }
  }
}
