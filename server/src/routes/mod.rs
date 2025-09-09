use std::{io::Write, net::TcpStream};

use http::{header, request::Request, response::{Response, Serialize}};
use tracing::trace;

pub(crate) mod tree;


// Router
//  |
//  | push
//  |  /             -> root()
//  |  /home         -> home()
//  |  /:id/profile  -> profile(id)
//
//  /1/2/3/4
//
//  tree = self.tree()
//  loop --
//  tree = tree.get(1)
//  tree = tree
//

pub struct Router {
}

impl Router {
  pub(crate) fn route<'a>(&mut self, mut stream : TcpStream, req : Request ) {

    match req.get_uri() {
      "/" => {
        let res = Response::builder()
          .insert(header::field::CONNECTION, "Close".to_owned())
          .status(200)
          .add_body(b"hello!?");

        let _ = stream.write(&res.serialize()).map_err(|i| {
          trace!("disconnected on write: {:?}",i.kind());
        });

        let _ = stream.flush().map_err(|i| {
          trace!("disconnected on flush: {:?}",i.kind());
        });

      }





      _ => {
        let res = Response::builder()
          .insert(header::field::CONNECTION, "Close".to_owned())
          .insert(header::field::LOCATION, "/".to_owned())
          .status(301);

        let _ = stream.write(&res.serialize()).map_err(|i| {
          trace!("disconnected on write: {:?}",i.kind());
        });

        let _ = stream.flush().map_err(|i| {
          trace!("disconnected on flush: {:?}",i.kind());
        });
      }
    }
  }
}

