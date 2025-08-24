use std::collections::HashMap;

#[derive(Default)]
pub struct Segment<'a> {
  tree: HashMap<&'a str, SegmentType<'a>>,
}

pub enum SegmentType<'a> {
  SubTree(Segment<'a>),
  FUNC(fn()),
  ID(&'a str),
}

pub enum UriType<'a> {
  Path(&'a str),
  ID(&'a str),
}

impl<'a> Segment<'a> {
  fn parser_uri(uri: &str) -> Vec<UriType<'_>> {
    match uri.trim_matches('/') {
      "" => {
        vec![UriType::Path(uri)]
      }

      uri => {
        let uri = uri
          .split('/')
          .map(|i| {
            if i.starts_with(':') {
              UriType::ID(i.trim_matches(':'))
            } else {
              UriType::Path(i)
            }
          })
          .collect();

        uri
      }
    }
  }

  fn merge(mut self, key: &'a str, rhs: Self) -> Self {

    let uri = Self::parser_uri(key);

    loop {
      let mut i = uri.iter().peekable();
      match i.next() {
        Some(UriType::Path(a)) => {
          
        }
        Some(UriType::ID(a)) => {}

        _ => {
          break;
        }
      }
    }

    self.tree.insert(key, SegmentType::SubTree(rhs));
    self
  }

  fn add_method(mut self, key: &'a str, func: fn()) -> Self {
    self.tree.insert(key, SegmentType::FUNC(func));
    self
  }
}

