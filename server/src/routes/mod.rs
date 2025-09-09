use std::{io::Write, net::TcpStream};

use http::{
  header,
  request::Request,
  response::{Response, Serialize},
};
use tracing::{info, trace, warn};

use crate::git;

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

pub struct Router;

impl Router {
  pub(crate) fn route(&mut self, mut stream: TcpStream, req: Request<'_>) {
    let uri = req.get_uri();
    match uri {
      "/" => {
        let res = Response::builder()
          .insert(header::field::CONNECTION, "Close".to_owned())
          .status(200)
          .add_body(b"hello!?");

        let _ = stream.write(&res.serialize()).map_err(|i| {
          trace!("disconnected on write: {:?}", i.kind());
        });

        let _ = stream.flush().map_err(|i| {
          trace!("disconnected on flush: {:?}", i.kind());
        });
      }

      uri => {
        trace!("header : {:?}", req.header);
        let is_git =  req.header.map.map(|i| {
          i.get(header::field::USER_AGENT).map(|i| {
            trace!("user agent = {}", i);
            i.trim().starts_with("git/")
          }).unwrap_or(false)
        }).unwrap_or(false);
        

        if is_git {
          trace!("git http_backend detected");

          if let Some(query) = req.header.uri.raw {
            let method = req.header.method;
            let path_info = uri;
            git::http_backend(&method, path_info, query, "/srv/git", &mut stream);

          } else {
            info!("command not provided >> status 400 client error");
            let res = Response::builder()
              .insert(header::field::CONNECTION, "Close".to_owned())
              .status(400);
            let _ = stream.write(&res.serialize()).map_err(|i| {
              trace!("disconnected on write {:?}", i.kind());
            });
            let _ = stream.flush().map_err(|i| {
              trace!("disconnected on flush: {:?}", i.kind());
            });
          }

        } else {

          trace!("default uri handler");
          let res = Response::builder()
            .insert(header::field::CONNECTION, "Close".to_owned())
            .insert(header::field::LOCATION, "/".to_owned())
            .status(301);

          let _ = stream.write(&res.serialize()).map_err(|i| {
            trace!("disconnected on write: {:?}", i.kind());
          });

          let _ = stream.flush().map_err(|i| {
            trace!("disconnected on flush: {:?}", i.kind());
          });
        }
      }
    }
  }
}
