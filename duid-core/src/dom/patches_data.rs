use crate::arena::{ArenaNode, NodeId, Pairs};
use std::collections::HashSet;


#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Patch<MSG> 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
{
    // (old_id, new_id, new_nodes_ids, new_nodes)
    Replacing(NodeId, NodeId, HashSet<Pairs>, Vec<ArenaNode<MSG>>),
    // (old_id, value)
    ValueChanged(NodeId, Option<String>),
    Removed(NodeId)
}


#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum DataState {
    Tag,
    Value,
    #[default]
    None,
}