use web_sys::{Element, Node, Document};
use std::rc::Rc;
use std::cell::RefCell;
use crate::{
    core::duid_events::Dispatch,
    arena::{ArenaNode, Arena},
    dom::HtmlNodeBuilder
};
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct NodeId {
    pub(crate) value: usize
}


impl NodeId {
    pub(crate) fn new(value: usize) -> NodeId {
        NodeId {
            value
        }
    }

    pub(crate) fn get_node_by_id_mut<'a, MSG>(&'a self, arena: &'a mut Arena<ArenaNode<MSG>>) -> Option<&mut ArenaNode<MSG>> {
        arena.nodes.iter_mut().find(|node| node.id == *self)    
    }

    pub(crate) fn get_children(&self, ids: &[[NodeId; 2]]) -> Vec<NodeId> {
        ids.iter().filter(|id| id[0] == *self).map(|i| i[1].clone()).collect()  
    }

    pub(crate) fn get_levels(
        &self,
        level: usize, 
        level_nodes: &[NodeId], 
        values: &mut Vec<(usize, Vec<NodeId>)>,
        ids: &[[NodeId; 2]]
    ) {
        values.push((level, level_nodes.clone().to_vec()));

        // get all nodes that have parent in level
        let mut local_level_nodes: Vec<_> = vec![];
        level_nodes.iter().for_each(|node| {
            local_level_nodes.extend_from_slice(&node.get_children(&ids));
        });

        if local_level_nodes.len() > 0 {
            self.get_levels(
                level + 1, 
                &local_level_nodes,
                values,
                &ids
            );
        }
    }
}
