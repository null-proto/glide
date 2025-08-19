use std::net::TcpListener;
use server::serve;


fn main() {
  let lisner = TcpListener::bind("[::]:8000").unwrap();
  serve(lisner);
}
