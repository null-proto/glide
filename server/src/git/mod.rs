use std::io::Stdout;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;

use tracing::{info, trace};

#[allow(unused_mut)]
pub fn http_backend<'a>(
  env : Vec<(&str,&str)>,
  method: &'a str,
  path_info: &'a str,
  query: &'a str,
  project_dir: &'static str,
) -> Option<String> {
  trace!(
    "REQUEST_METHOD: {} GIT_PROJECT_ROOT: {} PATH_INFO: {} QUERY_STRING: {}",
    method, project_dir, path_info, query
  );
  let mut buf = String::new();

  let mut _git = Command::new("git")
    .current_dir(project_dir)
    .arg("http-backend")
    .env("REQUEST_METHOD", method)
    .env("GIT_PROJECT_ROOT", project_dir)
    .env("PATH_INFO", path_info)
    .env("GIT_HTTP_EXPORT_ALL", "1")
    .env("QUERY_STRING", query)
    .envs(env)
    .stdout(Stdio::piped())
    .spawn()
    .ok()?;

  let mut stdout = _git.stdout.take().unwrap();
  _ = stdout.read_to_string(&mut buf);
  trace!("git http-backend [{}]" , _git.wait().unwrap());
  trace!("read : {}" ,buf);

  Some(buf)
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
