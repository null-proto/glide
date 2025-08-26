use std::net::TcpListener;
use server::serve;
use tracing::{level_filters::LevelFilter, trace};


fn main() {
  let tracer = tracing_subscriber::FmtSubscriber::builder()
    .with_max_level(LevelFilter::TRACE)
    .finish();

  let _ = tracing::subscriber::set_global_default(tracer);

  trace!("listening on [::]:8000");
  let lisner = TcpListener::bind("[::]:8000").unwrap();
  serve(lisner);
}
