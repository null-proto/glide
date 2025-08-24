use std::net::TcpStream;


pub struct Request {
  header: http::HeaderMap,
  stream: TcpStream,
}
