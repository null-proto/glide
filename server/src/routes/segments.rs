use std::collections::HashMap;

#[derive(Default)]
pub struct Segment<'a> {
  pub(crate) tree: HashMap<&'a str, SegmentType<'a>>,
}

pub enum SegmentType<'a> {
  SubTree(Segment<'a>),
  FUNC(fn()),
  Path(&'a str),
  ID(&'a str),
}

pub enum UriType<'a> {
  Path(&'a str),
  ID(&'a str),
}

trait Builder<'a> {
  fn parser_uri(uri: &str) -> Vec<UriType<'_>>;
  fn merge(self, key: &'a str, rhs: Self) -> Self;
  fn add_method(self, key: &'a str, func: fn()) -> Self;
}

impl<'a> Builder<'a> for Segment<'a> {
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

    let tree = &self.tree;

    for i in uri.iter() {
      match i {
        UriType::Path(a) => {
          tree.get(a)
        },

        UriType::ID(a) => {}
      }
    }

    // for i in uri.iter() {
    //   match i {
    //     UriType::Path(a) => {
    //       if let Some(k) = tree.get_mut(a) {
    //         *k = SegmentType::SubTree(Segment::default());
    //       } else {
    //         tree.insert(a, SegmentType::SubTree(Segment::default()));
    //       }
    //     }
    //
    //     UriType::ID(a) => {
    //       if let Some(k) = tree.get_mut(a) {
    //         *k = SegmentType::SubTree(Segment::default());
    //       } else {
    //         tree.insert(a, SegmentType::SubTree(Segment::default()));
    //       }
    //     }
    //   }
    // }

    self.tree.insert(key, SegmentType::SubTree(rhs));
    self
  }

  fn add_method(mut self, key: &'a str, func: fn()) -> Self {
    self.tree.insert(key, SegmentType::FUNC(func));
    self
  }
}
