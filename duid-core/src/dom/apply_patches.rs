/*
use crate::core::{
    v_node::VirtualNode
};

pub(crate) fn apply_patches<MSG>(old_node: &mut VirtualNode<MSG>, new_node: &VirtualNode<MSG>, patches: &[Patch<MSG>])
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    mark_changes(&mut old_node, &new_node, &patches);
}

pub(crate) fn mark_changes<MSG>(old_node: &mut VirtualNode<MSG>, new_node: &VirtualNode<MSG>, patches: &[Patch<MSG>])
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    
    match patches.iter() {
        Patch::AddedNodes(nodes) => {

        },
        Patch::RemovedNodes(nodes) => {

        }
    }

    let mut children = self.children.iter_mut();
    
    while let Some(child) = children.next() {
        local_key = child.set_key(local_key + 1);
    }
}
*/