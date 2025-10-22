use std::net::TcpListener;
use std::io::Write;

use http::response::Response;
use http::request::Request;
use http::header::field;
use tracing::warn;
use tracing::trace;
use tracing::info;
use tracing::debug;

use crate::config::ServerConfig;

pub mod git;
pub mod config;
pub mod err;

pub fn serve(listener: TcpListener , config : ServerConfig) {

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
                git::http_backend(env, req.header.method(), i, uri.query_str().unwrap(), &config.root_dir)
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
