use tokio::net::TcpListener;
use tokio::io::{AsyncWrite , AsyncWriteExt};
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

pub async fn serve(listener: TcpListener , config : ServerConfig) {

  while let Ok((mut stream, peer)) = listener.accept().await {
    info!("connection form : {}", peer);

    match Request::new(&mut stream).await {
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
                git::http_backend(env, req.header.method(), i, uri.query_str().unwrap(), &config.root_dir).await
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

              stream.write_all(&res.get()).await;
            } else {
              let res = Response::build()
                .status(308)
                .status_text("Permanent Redirect")
                .header(field::LOCATION, i.trim_end_matches("/info/refs"))
                .body(b"Only allowed git > 2.x")
                .finish();
              stream.write_all(&res.get()).await;
            }
          }

          _ => {
            let res = Response::build()
              .status(404)
              .status_text("Not Found")
              .finish();
            stream.write_all(&res.get()).await;
          }
        }
      }

      Err(e) => {
        warn!("request cancelled {}, cuz {:?}", peer, e);
      }
    }
  }
}
