use std::{net::TcpStream, os::fd::{AsRawFd, FromRawFd}, process::{Command, Stdio}};


pub fn http_backend<'a>(
  method: &'a str,
  path_info: &'a str,
  query: &'a str,
  project_dir: &'static str,
  stream : &mut TcpStream
) {
  let stdin = unsafe { Stdio::from_raw_fd( stream.as_raw_fd()) };
  let stdout = unsafe { Stdio::from_raw_fd( stream.as_raw_fd()) };

  let git = Command::new("git http-backend")
    .env("REQUEST_METHOD", method)
    .env("GIT_PROJECT_ROOT", project_dir)
    .env("PATH_INFO", path_info)
    .env("GIT_HTTP_EXPORT_ALL", "1")
    .env("QUERY_STRING", query)
    .stdout(stdout)
    .stdin(stdin)
    .spawn();

  match git {

    _ => {}
  }

}


pub fn create_bare<'a>(
  path_info: &'a str,
  project_dir: &'static str,
) -> Result<() , std::io::Error> {

  let path = format!("{}/{}" ,project_dir , path_info );

  let _ = Command::new(path).spawn()?;
  let _ = Command::new("git init -m master --bare").spawn()?;

  Ok(())
}
