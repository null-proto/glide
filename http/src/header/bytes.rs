use std::{collections::HashMap, fmt::Display, hash::Hash, sync::Arc};

#[derive(Debug, Clone)]
pub struct Bytes(Arc<[u8]>, usize, usize);

#[derive(Debug, Clone)]
pub enum ByteType {
  Bytes(Bytes),
  Str(&'static str),
}

pub type ByteMap = HashMap<ByteType, Bytes>;

pub trait TryStr {
  fn try_str<'a>(&'a self) -> Option<&'a str>;
}

pub trait TryRaw {
  fn try_raw<'a>(&'a self) -> Option<&'a [u8]>;
}

unsafe impl Sync for Bytes {}
unsafe impl Send for Bytes {}

impl Bytes {
  pub fn new(arr: &Arc<[u8]>, start: usize, end: usize) -> Self {
    Self(arr.clone(), start, end)
  }

  pub fn from(arr: Arc<[u8]>, start: usize, end: usize) -> Self {
    Self(arr, start, end)
  }
}

impl Bytes {
  pub fn size(&self) -> usize {
    self.0.len()
  }

  pub fn len(&self) -> usize {
    self.2 - self.1
  }

  pub fn slice<'a>(&self) -> &[u8] {
    &self.0[self.1..self.2]
  }
}

impl TryStr for Bytes {
  fn try_str<'a>(&'a self) -> Option<&'a str> {
    core::str::from_utf8(self.0.get(self.1..self.2)?).ok()
  }
}

impl TryRaw for Bytes {
  fn try_raw<'a>(&'a self) -> Option<&'a [u8]> {
    self.0.get(self.1..self.2)
  }
}

impl TryStr for ByteType {
  fn try_str<'a>(&'a self) -> Option<&'a str> {
    match self {
      Self::Bytes(a) => a.try_str(),
      Self::Str(a) => Some(a),
    }
  }
}

impl TryRaw for ByteType {
  fn try_raw<'a>(&'a self) -> Option<&'a [u8]> {
    match self {
      Self::Bytes(a) => a.try_raw(),
      Self::Str(a) => Some(a.as_bytes()),
    }
  }
}

impl PartialEq for Bytes {
  fn eq(&self, other: &Self) -> bool {
    if let Some(a) = self.try_raw() {
      if let Some(b) = other.try_raw() {
        a == b
      } else {
        false
      }
    } else {
      false
    }
  }
}

impl PartialEq for ByteType {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Self::Str(a) => match other {
        Self::Str(b) => a == b,
        Self::Bytes(b) => b.try_raw().map(|i| *i == *a.as_bytes()).unwrap_or(false),
      },

      Self::Bytes(a) => match other {
        Self::Str(b) => a.try_raw().map(|i| *i == *b.as_bytes()).unwrap_or(false),
        Self::Bytes(b) => a == b,
      },
    }
  }
}

impl Eq for Bytes {}
impl Eq for ByteType {}

impl Hash for Bytes {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    if let Some(s) = self.try_str() {
      s.hash(state);
    } else {
      '\r'.hash(state);
    }
  }
}

impl Hash for ByteType {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    if let Some(s) = self.try_str() {
      s.hash(state);
    } else {
      '\r'.hash(state);
    }
  }
}

impl Display for Bytes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.try_str().unwrap_or("<err>"))
  }
}

impl Display for ByteType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.try_str().unwrap_or("<err>"))
  }
}

impl<'a> Into<ByteType> for Bytes {
  fn into(self) -> ByteType {
    ByteType::Bytes(self)
  }
}

impl From<&'static str> for ByteType {
  fn from(value: &'static str) -> Self {
    ByteType::Str(value)
  }
}
