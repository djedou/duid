use crate::{
    arena::{ArenaNode, Arena, ArenaNodeState, Pairs}
};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
pub(crate) struct NodeId {
    pub(crate) level: usize,
    pub(crate) level_index: usize,
    pub(crate) global_index: usize
}

impl PartialEq<NodeId> for NodeId {
    fn eq(&self, other: &NodeId) -> bool {
        self.level == other.level &&
        self.level_index == other.level_index &&
        self.global_index == other.global_index
    }
}

impl Eq for NodeId {}

impl Hash for NodeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.level.hash(state);
        self.level_index.hash(state);
        self.global_index.hash(state);
    }
}



impl NodeId {
    pub(crate) fn new(level: usize, level_index: usize, global_index: usize) -> NodeId {
        NodeId {
            level,
            level_index,
            global_index
        }
    }

    pub(crate) fn from_global_index(global_index: usize) -> NodeId {
        NodeId {
            level: 0,
            level_index: 0,
            global_index
        }
    }

    pub(crate) fn get_duid_id(&self) -> String {
        format!("{}-{}-{}", self.level, self.level_index, self.global_index)
    }

    pub(crate) fn set_node_id(
        level: usize,
        level_nodes: &[Pairs],
        node_ids: &mut HashSet<Pairs>, 
        index_pairs: &[[NodeId; 2]]
    ) {
        level_nodes.iter().for_each(|node| {node_ids.insert(node.clone());});
        
        // get all nodes that have parent in level
        let mut local_level_nodes: Vec<_> = vec![];
        level_nodes.iter().for_each(|pairs| {
            let children: Vec<_> = 
                index_pairs.iter().filter(|ids| ids[0].global_index == pairs.child.global_index).map(|i| {
                    Pairs::new(
                        pairs.child.clone(), // parent
                        i[1].clone(), // child
                    )
                }).collect();
            
            local_level_nodes.extend_from_slice(&children);
        });

        local_level_nodes.iter_mut().enumerate().for_each(|(index, node_id)| {
            node_id.child.level_index = index;
            node_id.child.level = level;
        });

        local_level_nodes.sort_by(|a, b| a.child.global_index.cmp(&b.child.global_index));

        if local_level_nodes.len() > 0 {
            NodeId::set_node_id(
                level + 1,
                &local_level_nodes,
                node_ids, 
                &index_pairs
            );
        }
    }

    pub(crate) fn get_node_by_id_mut<'a, MSG>(&'a self, arena: &'a mut Arena<ArenaNode<MSG>>) -> Option<&mut ArenaNode<MSG>> 
    where 
        MSG: Clone
    {
        arena.nodes.iter_mut().find(|node| node.id == *self && node.node_state != ArenaNodeState::Removed)    
    }

    pub(crate) fn get_children(&self, ids: &HashSet<Pairs>) -> Vec<NodeId> {
        let mut children: Vec<_> = ids.iter()
            .filter(|id| id.parent == *self )
            .map(|i| i.child.clone())
            .collect();

        children.sort_by(|a, b| a.global_index.cmp(&b.global_index));

        children
    }

    pub(crate) fn get_parent(&self, ids: &HashSet<Pairs>) -> Option<NodeId> {
        ids.iter().find(|id| id.child == *self).map(|i| i.parent.clone())
    }

    pub(crate) fn get_node_by_id<'a, MSG>(&'a self, arena: &'a Arena<ArenaNode<MSG>>) -> Option<&ArenaNode<MSG>> 
    where 
        MSG: Clone
    {
        arena.nodes.iter().find(|node| node.id == *self && node.node_state != ArenaNodeState::Removed)
    }

    pub(crate) fn get_index_in_parent_children(&self, ids: &HashSet<Pairs>) -> Option<(NodeId, usize)> {
        let Some(parent_node_id) = self.get_parent(&ids) else {
            return None;
        };

        let Some(index) = &parent_node_id.get_children(&ids).iter().position(|r| r == self) else {
            return None;
        };
        
        Some((parent_node_id, *index))
    }

/*

    pub(crate) fn get_node_by_id_to_patch<'a, MSG>(&'a self, arena: &'a Arena<ArenaNode<MSG>>) -> Option<&ArenaNode<MSG>> 
    where 
        MSG: Clone
    {
        arena.nodes.iter().find(|node| 
            node.id == *self &&
            (node.node_state != ArenaNodeState::Removed ||
            node.node_state != ArenaNodeState::UnChanged)
        ) 
    }

    pub(crate) fn get_pair_mut<'a>(&'a self, ids: &'a mut [[NodeId; 2]]) -> Option<&mut [NodeId; 2]> {
        ids.iter_mut().find(|id| id[1] == *self)
    }

    
    */
}
