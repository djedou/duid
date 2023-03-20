use crate::{
    arena::{ArenaNode, Arena, ArenaNodeState}
};

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

    pub(crate) fn get_node_by_id_mut<'a, MSG>(&'a self, arena: &'a mut Arena<ArenaNode<MSG>>) -> Option<&mut ArenaNode<MSG>> 
    where 
        MSG: Clone
    {
        arena.nodes.iter_mut().find(|node| node.id == *self && node.node_state != ArenaNodeState::Removed)    
    }

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

    pub(crate) fn get_node_by_id<'a, MSG>(&'a self, arena: &'a Arena<ArenaNode<MSG>>) -> Option<&ArenaNode<MSG>> 
    where 
        MSG: Clone
    {
        arena.nodes.iter().find(|node| node.id == *self&& node.node_state != ArenaNodeState::Removed)
    }

    pub(crate) fn get_children(&self, ids: &[[NodeId; 2]]) -> Vec<NodeId> {
        ids.iter().filter(|id| id[0] == *self).map(|i| i[1].clone()).collect()  
    }

    pub(crate) fn get_parent(&self, ids: &[[NodeId; 2]]) -> Option<NodeId> {
        ids.iter().find(|id| id[1] == *self).map(|i| i[0].clone())
    }

    pub(crate) fn get_pair_mut<'a>(&'a self, ids: &'a mut [[NodeId; 2]]) -> Option<&mut [NodeId; 2]> {
        ids.iter_mut().find(|id| id[1] == *self)
    }

    pub(crate) fn get_index_in_parent_children(&self, ids: &[[NodeId; 2]]) -> Option<(NodeId, usize)> {
        let Some(parent_node_id) = self.get_parent(&ids) else {
            return None;
        };

        let Some(index) = &parent_node_id.get_children(&ids).iter().position(|r| r == self) else {
            return None;
        };
        
        Some((parent_node_id, *index))
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

        local_level_nodes.sort_by(|a, b| a.value.cmp(&b.value));
        local_level_nodes.dedup();

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
