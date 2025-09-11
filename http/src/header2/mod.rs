use crate::header::{uri::Uri, Method};

pub mod bytes;
pub mod uri;

struct Header<'a>( Method , Uri<'a> );
