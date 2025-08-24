use std::net::TcpListener;

pub mod git;
pub mod preludes;
pub mod request;
pub mod routes;

pub fn serve(listener: TcpListener) {
  while let Ok(mut stream) = listener.accept() {
  }
}
