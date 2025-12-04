use std::process::exit;
use glide::cfg::ConfigBuilder;
use tokio::net::TcpListener;

use server::serve;
use tracing::trace;

use crate::cli::Cli;

pub fn init(cli: Cli) {
  let rt = tokio::runtime::Builder::new_current_thread()
    .enable_io()
    .build()
    .unwrap();

  match ConfigBuilder::new()
    .set_repo_dir(cli.project_root.into())
    .finish() {

    Ok(cfg) => {
      trace!("listening on [::]:8000");

      rt.block_on(async {
        let lisner = TcpListener::bind("[::]:8000").await.unwrap();
        serve(lisner, cfg.repo_dir).await;
      })
    }

    Err(e) => {
      eprintln!("{}", e);
      exit(1);
    }
  }
}
