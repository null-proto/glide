use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub mod git;
pub mod preludes;


pub fn serve(listener: TcpListener) {
  while let Ok(mut stream) = listener.accept() {
    let req: Result<http::HeaderMap, http::error::Error> = preludes::read_request(&mut stream.0);
    println!("{:?}\n\n", req);
  }
}
