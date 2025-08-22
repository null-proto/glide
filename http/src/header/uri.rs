use std::{collections::HashMap, path, str::FromStr};

use crate::{error::Error, header::Parse};

#[derive(Default, Debug, Clone)]
pub struct Uri<'a> {
  path: &'a str,
  query: Option<HashMap<&'a str, &'a str>>,
}

impl<'a> Parse<'a> for Uri<'a> {
  fn parse(s: &'a [u8]) -> Result<Self, crate::error::Error>
  where
    Self: Sized,
  {
    let s = str::from_utf8(s).map_err(|_| Error::UriParse)?.trim();
    match s.split_once('?') {
      Some((path, query_str)) => {
        let a: Option<HashMap<&'a str, &'a str>> = if !query_str.is_empty() {
          Some(
            query_str
              .split('&')
              .filter_map(|i| {
                if !i.is_empty() {
                  Some(i.split_once('=').unwrap_or((i, "")))
                } else {
                  None
                }
              })
              .collect(),
          )
        } else {
          None
        };

        Ok(Self {
          path: path,
          query: a,
        })
      }
      None => Ok(Self {
        path: s,
        query: None,
      }),
    }
  }
}

#[cfg(test)]
mod unit_test {
  use std::{io::Bytes, sync::Arc};

  use super::Uri;
  use crate::error;
  use crate::header::Parse;

  #[test]
  fn test_uri() {
    let uri: Arc<[u8]> = Arc::from("/home/user/resource.html?k1=v1&k2=v2".as_bytes());
    let uri = Uri::parse(&uri);
    println!("URI: {:?}", uri.unwrap());

    let uri: Arc<[u8]> = Arc::from("/home/user/resource.html?k1=v1&k2".as_bytes());
    let uri = Uri::parse(&uri);
    println!("URI: {:?}", uri.unwrap());

    let uri: Arc<[u8]> = Arc::from("/home/user/resource.html?".as_bytes());
    let uri = Uri::parse(&uri);
    println!("URI: {:?}", uri.unwrap());

    let uri: Arc<[u8]> = Arc::from("/home/user/resource.html".as_bytes());
    let uri = Uri::parse(&uri);
    println!("URI: {:?}", uri.unwrap());
  }
}
