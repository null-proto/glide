use std::process::exit;
use tokio::net::TcpListener;

use server::{config::ServerConfig, serve};
use tracing::trace;

use crate::cli::Cli;

pub fn init(cli: Cli) {
  let rt = tokio::runtime::Builder::new_current_thread()
    .build()
    .unwrap();

  match ServerConfig::new(cli.project_root) {
    Ok(a) => {
      trace!("listening on [::]:8000");

      rt.block_on(async {
        let lisner = TcpListener::bind("[::]:8000").await.unwrap();
        serve(lisner, a).await;
      })
    }

    Err(e) => {
      eprintln!("{}", e);
      exit(1);
    }
  }
}
