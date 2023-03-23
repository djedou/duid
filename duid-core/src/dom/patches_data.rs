use crate::arena::{Arena, ArenaNode, NodeId, ArenaNodeState, ArenaIterator, Pairs};
use std::collections::HashSet;
use crate::core::{
    html::attributes::Attribute
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Patch<MSG> 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
{   
    // (old, new)
    IdChanged(NodeId, NodeId), 
    // (old_id, new_id, new_nodes_ids, new_nodes)
    Replacing(NodeId, NodeId, HashSet<Pairs>, Vec<ArenaNode<MSG>>),
    // (old_id, new_id, value)
    ValueChanged(NodeId, NodeId, Option<String>),
    // (old_id, new_id, props)
    PropsChanged(NodeId, NodeId, Vec<Attribute<MSG>>),
    Removed(NodeId),
}


#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum DataState {
    Tag,
    Value,
    Namespace,
    Props,
    ChildrenSize,
    GlobalIndex,
    #[default]
    None,
}