// #![allow(unused)]

use std::{collections::HashMap, fmt::Display, hash::Hash, sync::Arc};

#[derive(Debug , Clone)]
pub struct Bytes(Arc<[u8]>, usize, usize);

#[derive(Debug , Clone)]
pub enum ByteType<'a> {
  Bytes(Bytes),
  Str(&'a str),
}

pub type ByteMap<'a> = HashMap<ByteType<'a>, Bytes>;

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
}

impl TryStr for Bytes {
  fn try_str<'a>(&'a self) -> Option<&'a str> {
    str::from_utf8(self.0.get(self.1..self.2)?).ok()
  }
}

impl TryRaw for Bytes {
  fn try_raw<'a>(&'a self) -> Option<&'a [u8]> {
    self.0.get(self.1..self.2)
  }
}

impl TryStr for ByteType<'_> {
  fn try_str<'a>(&'a self) -> Option<&'a str> {
    match self {
      Self::Bytes(a) => a.try_str(),
      Self::Str(a) => Some(a),
    }
  }
}

impl TryRaw for ByteType<'_> {
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

impl PartialEq for ByteType<'_> {
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
impl Eq for ByteType<'_> {}

impl Hash for Bytes {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    if let Some(s) = self.try_str() {
      s.hash(state);
    } else {
      '\r'.hash(state);
    }
  }
}


impl Hash for ByteType<'_> {
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
    write!(f , "{}" ,self.try_str().unwrap_or("<err>") )
  }
}

impl Display for ByteType<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f , "{}" , self.try_str().unwrap_or("<err>"))
  }
}

impl<'a> Into<ByteType<'a>> for Bytes {
  fn into(self) -> ByteType<'a> {
    ByteType::Bytes(self)
  }
}

impl<'a> From<&'a str> for ByteType<'a> {
  fn from(value: &'a str) -> Self {
    ByteType::Str(value)
  }
}
