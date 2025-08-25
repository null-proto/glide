use std::ops::Deref;

pub struct Status(pub(crate) u16);

impl Deref for Status {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    match self.0 {
      i if i < 200 => "Not Implemanted",
      i if i >= 200 && i < 300 => "Ok",
      i if i >= 300 && i < 400 => "Redirect",
      i if i >= 400 && i < 500 => "Bad",
      _ => "Server fault",
    }
  }
}

