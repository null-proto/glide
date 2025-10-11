use std::{io::Write, net::TcpListener};

use http::{
  header::field,
  request::{self, Request},
  response::{Response, ResponseBuilder},
};
use tracing::{debug, info, trace, warn};

pub mod git;

pub fn serve(listener: TcpListener) {
  while let Ok((mut stream, peer)) = listener.accept() {
    info!("connection form : {}", peer);

    match Request::new(&mut stream) {
      Ok(req) => {
        trace!("{}", req);

        let uri = req.header.uri();
        let client = req.header.get(field::USER_AGENT).unwrap_or("noclient");
        debug!("{}", uri);

        match uri.path() {
          Some(i) if i.ends_with("/info/refs") => {
            if client.starts_with("git/") {
              // here is the git client
              let env = req.header.gather();
              let res = if let Some(git_res) =
                git::http_backend(env, req.header.method(), i, uri.query_str().unwrap(), ".")
              {
                trace!("git_res : {}",git_res);
                Response::build()
                  .status(200)
                  .status_text("Ok")
                  .attach_raw(git_res.as_bytes())
              } else {
                warn!("> git http error");
                Response::build()
                  .status(404)
                  .status_text("Not Found")
                  .body(b"Repo Not Exists")
                  .finish()
              };

              _ = stream.write_all(&res.get());
            } else {
              let res = Response::build()
                .status(308)
                .status_text("Permanent Redirect")
                .header(field::LOCATION, i.trim_end_matches("/info/refs"))
                .body(b"Only allowed git > 2.x")
                .finish();
              _ = stream.write_all(&res.get());
            }
          }

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
        warn!("request cancelled {}, cuz {:?}", peer, e);
      }
    }
  }
}
