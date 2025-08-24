use http::header::HeaderMap;
use std::net::TcpListener;

pub mod git;
pub mod preludes;


pub fn serve(listener: TcpListener) {
  while let Ok(mut stream) = listener.accept() {
  }
}
