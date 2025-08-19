use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn read_request(stream: &mut TcpStream) -> String {
    let mut buf = [0; 512];
    let mut req = String::new();

    'outer: loop {
        match stream.read(&mut buf) {
            Ok(0) => {
              break 'outer;
            }
            Ok(n) => {
                req.push_str(str::from_utf8(&buf[..n]).expect("convert to utf8"));
                if buf[n-4..n].eq(&[0x0D,0x0A,0x0D,0x0A]) {
                  break;
                }
            }
            Err(_) => {
              break;
            }
        }
    };

    req
}

pub fn serve(listener: TcpListener) {
  while let Ok(mut stream) = listener.accept() {
    let req = read_request(&mut stream.0);
    println!("{}",req);
  }
}
