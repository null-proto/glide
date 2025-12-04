use std::{
  io::ErrorKind,
  net::{IpAddr, Ipv4Addr},
  path::PathBuf,
  str::FromStr,
};

pub struct Config {
  pub address: IpAddr,
  pub port: u32,
  pub repo_dir: PathBuf,
}

pub struct ConfigBuilder {
  address: Option<IpAddr>,
  port: Option<u32>,
  repo_dir: Option<PathBuf>,
}

impl ConfigBuilder {
  pub fn new() -> Self {
    Self {
      address: None,
      port: None,
      repo_dir: None,
    }
  }
}

impl ConfigBuilder {
  pub fn set_address(mut self, address: IpAddr) -> ConfigBuilder {
    self.address = Some(address);
    self
  }

  pub fn set_port(mut self, port: u32) -> ConfigBuilder {
    self.port = Some(port);
    self
  }

  pub fn set_repo_dir(mut self, path: PathBuf) -> ConfigBuilder {
    self.repo_dir = Some(path);
    self
  }
}

impl ConfigBuilder {
  fn validate(&self) -> Result<(), ErrorKind> {
    if let Some(path) = &self.repo_dir {
      if path.exists() {
        Ok(())
      } else {
        Err(ErrorKind::NotFound)
      }
    } else {
      Err(ErrorKind::NotFound)
    }
  }

  pub fn finish(self) -> Result<Config, ErrorKind> {
    if let Err(err) = self.validate() {
      Err(err)
    } else {
      Ok(Config {
        address: self
          .address
          .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        port: self.port.unwrap_or(8080),
        repo_dir: self
          .repo_dir
          .unwrap_or(PathBuf::from_str("/srv/git/").unwrap()),
      })
    }
  }
}
