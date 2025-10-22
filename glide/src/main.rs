use clap::Parser;
use tracing::level_filters::LevelFilter;

use crate::cli::Cli;

mod cli;
mod init;

fn main() {
  let tracer = tracing_subscriber::FmtSubscriber::builder()
    .with_max_level(LevelFilter::TRACE)
    .finish();

  let _ = tracing::subscriber::set_global_default(tracer);

  let cli = Cli::parse();
  init::init(cli);
}
