pub(crate) mod segments;


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

