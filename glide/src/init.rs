use std::{net::TcpListener, process};

use server::{config::ServerConfig, serve};
use tracing::trace;

use crate::cli::Cli;



pub fn init(cli : Cli) {
  match ServerConfig::new(cli.project_root) {
    Ok(a) => {
      trace!("listening on [::]:8000");
      let lisner = TcpListener::bind("[::]:8000").unwrap();
      serve(lisner, a);
    }

    Err(e) => {
      eprintln!("{}" , e);
      process::exit(1);
    }
  }

}
