use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum ServerError {
  ProjectRootNotFound(String),
  ProjectRootIsAFile(String),
}

impl Display for ServerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ProjectRootNotFound(a) => {
        write!(f , "ServerError: '{}' no such directory" , a)
      }
      Self::ProjectRootIsAFile(a) => {
        write!(f , "ServerError: '{}' cannot be a file" , a)
      }

    }
  }
}
