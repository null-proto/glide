use std::net::TcpStream;

use http::request::Request;

pub(crate) mod tree;


// Router
//  |
//  | push
//  |  /             -> root()
//  |  /home         -> home()
//  |  /:id/profile  -> profile(id)
//
//  /1/2/3/4
//
//  tree = self.tree()
//  loop --
//  tree = tree.get(1)
//  tree = tree
//

pub struct Router {
}

impl Router {
  fn route<'a>( stream : TcpStream, req : Request ) {

    match req.get_uri() {
      "/" => {

      }



      _ => {}
    }
  }
}

