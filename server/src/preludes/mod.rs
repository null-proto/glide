use std::{net::TcpStream, sync::Arc};
use std::io::Read;


pub fn read_request_bytes(stream: &mut TcpStream) -> Arc<[u8]> {
  let mut buf = [0; 512];
  let mut req = vec![];

  'outer: loop {
    match stream.read(&mut buf) {
      Ok(0) => {
        break 'outer;
      }
      Ok(n) => {
        req.extend_from_slice(&buf[..n]);
        if buf[n - 4..n].eq(&[0x0D, 0x0A, 0x0D, 0x0A]) {
          break;
        }
      }
      Err(_) => {
        break;
      }
    }
  }

  Arc::from(req)
}
