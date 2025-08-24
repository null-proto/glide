use std::net::TcpStream;
use http::header::HeaderMap;


pub struct Request<'a> {
  header: HeaderMap<'a>,
  stream: TcpStream,
}
