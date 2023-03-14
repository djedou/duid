use super::Indexes;
use crate::core::{
    v_node::VirtualNode
};

#[derive(Debug)]
pub(crate) enum Patch<MSG> {
    AddedBranches(Vec<Indexes<MSG>>),
    RemovedBranches(Vec<Indexes<MSG>>),
    //AddedNodes(Vec<VirtualNode<MSG>>),
    //RemovedNodes(Vec<VirtualNode<MSG>>)
}