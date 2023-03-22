use crate::arena::{Pairs, NodeId};
use std::collections::HashSet;

pub(crate) struct ArenaIterator {
    pub(crate) nodes_ids: HashSet<Pairs>
}

impl ArenaIterator {
    pub(crate) fn new(nodes_ids: HashSet<Pairs>) -> ArenaIterator {
        ArenaIterator {
            nodes_ids
        }
    }
}

impl<'a> IntoIterator for &'a ArenaIterator {
    type Item = NodeId;
    type IntoIter = ArenaIntoIterator<'a> ;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIntoIterator {
            arena_iterator: self,
            pointer: NodeId::new(1, 0, 1),
        }
    }
}

pub(crate) struct ArenaIntoIterator<'a>  {
    arena_iterator: &'a ArenaIterator,
    pointer: NodeId,
}

impl<'a> Iterator for ArenaIntoIterator<'a> {
    type Item = NodeId;
    fn next(&mut self) -> Option<NodeId> {
        // get all nodes for the level
        let mut level_nodes: Vec<_> = self.arena_iterator.nodes_ids.iter()
            .filter(|pairs| pairs.child.level == self.pointer.level)
            .collect();
        level_nodes.sort_by(|a, b| a.child.global_index.cmp(&b.child.global_index));

        // get node by index
        match level_nodes.get(self.pointer.level_index) {
            Some(pairs) => {
                // let set next pointer
                match level_nodes.get(self.pointer.level_index + 1) {
                    Some(next_pairs) => {
                        self.pointer = next_pairs.child.clone();
                    },
                    None => {
                        match self.arena_iterator.nodes_ids.iter()
                            .find(|pairs| 
                                pairs.child.level == self.pointer.level + 1 &&
                                pairs.child.level_index == 0
                            ) {
                                Some(n) => {
                                    self.pointer = n.child.clone();
                                },
                                None => {
                                    return None;
                                }
                            }
                    }
                }
                // return node
                Some(pairs.child.clone())
            },
            None => {
                None
            }
        }
    }
}
