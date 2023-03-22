mod arena;
mod node_id;
mod arena_node;
mod arena_iterator;

pub(crate) use arena::*;
pub(crate) use node_id::*;
pub(crate) use arena_node::*;
pub(crate) use arena_iterator::*;

use std::hash::{Hash, Hasher};
use std::collections::HashSet;


#[derive(Default, Debug, Clone)]
pub(crate) struct Pairs {
    pub(crate) parent: NodeId,
    pub(crate) child: NodeId
}

impl PartialEq<Pairs> for Pairs {
    fn eq(&self, other: &Pairs) -> bool {
        self.parent == other.parent &&
        self.child == other.child
    }
}

impl Eq for Pairs {}

impl Hash for Pairs {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        self.child.hash(state);
    }
}

impl Pairs {
    pub(crate) fn new(parent: NodeId, child: NodeId) -> Pairs {
        Pairs {
            parent,
            child
        }
    }
}