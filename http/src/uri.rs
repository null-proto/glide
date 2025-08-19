use std::str::FromStr;



#[derive(Default,Debug)]
pub struct Uri(Vec<String>);

impl FromStr for Uri {
  type Err = crate::error::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(
      Self(
        s.split_inclusive('/').map(|i| i.to_owned()).collect()
      )
    )
  }
}

#[cfg(test)]
mod unit_test {
    use crate::{error, uri::Uri};

  #[test]
  fn test_uri() {
    let uri : Result<Uri,error::Error> = "/home/user/resource.html".parse();
    println!("URI: {:?}",uri.unwrap());
  }
}
