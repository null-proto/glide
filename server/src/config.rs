use std::path::{Path, PathBuf};

use crate::err::{self, ServerError};

pub struct ServerConfig {
  pub root_dir : String
}

impl ServerConfig {
  fn validate_project_root(root_dir : String) -> Result<String , err::ServerError> {
    let path = PathBuf::from(&root_dir);

    if !path.exists() {
      return Err(err::ServerError::ProjectRootNotFound(root_dir))
    }

    if !path.is_dir() {
      return Err(err::ServerError::ProjectRootIsAFile(root_dir))
    }

    Ok(root_dir)
  }

  pub fn new(root_dir : String) -> Result<Self , ServerError> {
    let root_dir = Self::validate_project_root(root_dir)?;

    Ok(Self { root_dir })

  }
}
