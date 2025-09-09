use std::{io::Stdout, net::TcpStream, os::{fd::{AsRawFd, FromRawFd}, unix::process::CommandExt}, process::{Command, Stdio}};


pub fn http_backend<'a>(
  method: &'static str,
  path_info: &'a str,
  query: &'a str,
  project_dir: &'static str,
  stream : &mut TcpStream
) {
  let stdio = unsafe { Stdio::from_raw_fd( stream.as_raw_fd()) };

  let git = Command::new("git http-backend")
    .env("REQUEST_METHOD", method)
    .env("GIT_PROJECT_ROOT", project_dir)
    .env("PATH_INFO", path_info)
    .env("GIT_HTTP_EXPORT_ALL", "1")
    .env("QUERY_STRING", query)
    .stdout(stdio)
    .exec();

  match git.kind() {
    std::io::ErrorKind::NotFound => {}

    _ => {}
  }

}
