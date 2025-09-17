use std::{
  io,
  net::TcpStream,
  os::fd::AsFd,
  process::{Command, Stdio},
};

use tracing::{info, trace};

#[allow(unused_mut)]
pub fn http_backend<'a>(
  method: &'a str,
  path_info: &'a str,
  query: &'a str,
  project_dir: &'static str,
  mut stream: TcpStream,
) -> Result<(), io::Error> {
  trace!(
    "REQUEST_METHOD: {} GIT_PROJECT_ROOT: {} PATH_INFO: {} QUERY_STRING: {}",
    method, project_dir, path_info, query
  );

  let stdin = Stdio::from(stream.as_fd().try_clone_to_owned()?);
  let stdout = Stdio::from(stream.as_fd().try_clone_to_owned()?);

  let _git = Command::new("git")
    .current_dir(project_dir)
    .arg("http-backend")
    .env("REQUEST_METHOD", method)
    .env("GIT_PROJECT_ROOT", project_dir)
    .env("PATH_INFO", path_info)
    .env("GIT_HTTP_EXPORT_ALL", "1")
    .env("QUERY_STRING", query)
    .stdout(stdout)
    .stdin(stdin)
    .spawn()?;

  info!("sock :{:?}", stream.as_fd());

  Ok(())
}

pub fn create_bare<'a>(
  path_info: &'a str,
  project_dir: &'static str,
) -> Result<(), std::io::Error> {
  let path = format!("mkdir -p {}/{}", project_dir, path_info);

  let _ = Command::new(path).spawn()?;
  let _ = Command::new("git init -m master --bare").spawn()?;

  Ok(())
}
