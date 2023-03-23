use crate::arena::{Arena, ArenaNode, NodeId, ArenaNodeState, ArenaIterator, Pairs};
use super::{Patch, DataState};
use std::cmp::Ordering;
use std::collections::HashSet;


pub(crate) fn patches<MSG>(
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &mut Arena<ArenaNode<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    let arena_iterator = ArenaIterator::new(old_arena.nodes_ids.clone());

    let old_binding = old_arena.nodes_ids.clone();
    let new_binding = new_arena.nodes_ids.clone();
    let mut finded_new_ids: Vec<_> = new_binding.iter().map(|n| n.clone()).collect();

    let mut patches: Vec<_> = vec![];

    for node in &arena_iterator {
        patch_node(&node, old_arena, new_arena, &old_binding, &new_binding, &mut finded_new_ids, &mut patches);
    }
    let last_node = old_arena.get_last_id();
    patch_node(&last_node, old_arena, new_arena, &old_binding, &new_binding, &mut finded_new_ids, &mut patches);

    crate::console::info!("patches: {:#?}", patches);
}


fn patch_node<MSG>(
    old_id: &NodeId,
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &mut Arena<ArenaNode<MSG>>,
    old_nodes_ids: &HashSet<Pairs>,
    new_nodes_ids: &HashSet<Pairs>,
    new_nodes_ids_vec: &mut Vec<Pairs>,
    patches: &mut Vec<Patch<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    
    match new_nodes_ids_vec.iter_mut()
    .find(|new| 
        new.child.level == old_id.level && 
        new.child.level_index == old_id.level_index) 
        {
            Some(new_id) => {
                match (old_id.get_node_by_id_mut(old_arena), new_id.child.get_node_by_id_mut(new_arena)) {
                    (Some(old_node), Some(new_node)) => {
                        if old_node.node_state == ArenaNodeState::None {
                            old_node.node_state = ArenaNodeState::Visited;
                            new_node.node_state = ArenaNodeState::Visited;
                            let old_children_len = old_id.get_children(&old_nodes_ids).len();
                            let new_children_len = new_id.child.get_children(&new_nodes_ids).len();

                            match get_data_changed(
                                old_id.global_index, 
                                new_id.child.global_index,
                                old_children_len,
                                new_children_len,
                                &old_node, 
                                &new_node
                            ) {
                                DataState::Tag | DataState::Namespace | DataState::ChildrenSize => {
                                    old_node.node_state = ArenaNodeState::RemovedRoot;
                                    patches.push(Patch::RemovedRoot(old_id.clone()));
                                    mark_children_removed_state(&[old_id.clone()], old_arena, patches);

                                    let mut new_nodes_ids = HashSet::with_capacity(0);
                                    let mut new_nodes = vec![];
                                    mark_replacing_state(new_id.child.clone(), old_id.clone(), &mut new_nodes_ids, &mut new_nodes, new_arena); 
                                    mark_children_state(&[new_id.child.clone()], &mut new_nodes_ids, &mut new_nodes, new_arena);
                                    patches.push(Patch::Replacing(old_id.clone(), new_id.child.clone(), new_nodes_ids, new_nodes));
                                },
                                DataState::Value => {
                                    old_node.node_state = ArenaNodeState::Visited;
                                    new_node.node_state = ArenaNodeState::Visited;
                                    patches.push(Patch::ValueChanged(old_id.clone(), new_id.child.clone(), new_node.value.clone()));
                                },
                                DataState::Props => {
                                    old_node.node_state = ArenaNodeState::Visited;
                                    new_node.node_state = ArenaNodeState::Visited;
                                    patches.push(Patch::PropsChanged(old_id.clone(), new_id.child.clone(), new_node.props.clone()));
                                },
                                DataState::GlobalIndex => {
                                    old_node.node_state = ArenaNodeState::Visited;
                                    new_node.node_state = ArenaNodeState::Visited;
                                    patches.push(Patch::IdChanged(old_id.clone(), new_id.child.clone()));
                                },
                                DataState::None => {
                                    old_node.node_state = ArenaNodeState::Visited;
                                    new_node.node_state = ArenaNodeState::Visited;
                                }
                            }
                        }
                    },
                    _ => {}
                }
            },
            None => {
                match old_id.get_node_by_id_mut(old_arena) {
                    Some(old_node) => {
                        old_node.node_state = ArenaNodeState::RemovedRoot;
                        patches.push(Patch::RemovedRoot(old_id.clone()));
                        mark_children_removed_state(&[old_id.clone()], old_arena, patches);
                    },
                    _ => {}
                }
            }
        }
}

fn mark_replacing_state<MSG>(
    node: NodeId,
    old_id: NodeId,
    new_nodes_ids: &mut HashSet<Pairs>,
    new_nodes: &mut Vec<ArenaNode<MSG>>,
    new_arena: &mut Arena<ArenaNode<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
{
        match node.get_parent(&new_arena.nodes_ids) {
            Some(parent) => {
                match node.get_node_by_id_mut(new_arena) {
                    Some(child_node) => {
                        let mut new_child_node = child_node.clone();
                        child_node.node_state = ArenaNodeState::Visited;

                        new_child_node.node_state = ArenaNodeState::Visited;//Replacing(old_id.clone());
                        new_nodes.push(new_child_node);
                        new_nodes_ids.insert(Pairs::new(parent.clone(), node.clone()));
                    },
                    None => {}
                }
            },
            None => {}
        }
}


fn mark_children_state<MSG>(
    parents_nodes: &[NodeId], 
    new_nodes_ids: &mut HashSet<Pairs>,
    new_nodes: &mut Vec<ArenaNode<MSG>>,
    new_arena: &mut Arena<ArenaNode<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
{
    
    parents_nodes.iter().for_each(|parent| {
        let children = parent.get_children(&new_arena.nodes_ids);
        children.iter().for_each(|child| {
            match child.get_node_by_id_mut(new_arena) {
                Some(child_node) => {
                    child_node.node_state = ArenaNodeState::Visited;
                    new_nodes.push(child_node.clone());
                    new_nodes_ids.insert(Pairs::new(parent.clone(), child.clone()));
                },
                None => {}
            }
        });
        
        mark_children_state::<MSG>(&children, new_nodes_ids, new_nodes, new_arena);
    });
}

fn mark_children_removed_state<MSG>(
    parents_nodes: &[NodeId], 
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    patches: &mut Vec<Patch<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
{
    parents_nodes.iter().for_each(|parent| {
        let children = parent.get_children(&old_arena.nodes_ids);
        children.iter().for_each(|child| {
            match child.get_node_by_id_mut(old_arena) {
                Some(child_node) => {
                    if child_node.node_state == ArenaNodeState::None {
                        child_node.node_state = ArenaNodeState::Removed;
                        patches.push(Patch::Removed(child.clone()));
                    }
                },
                None => {}
            }
        });

        mark_children_removed_state::<MSG>(&children, old_arena, patches);
    });
}


fn get_data_changed<MSG>(
    old_global_index: usize, 
    new_global_index: usize,
    old_children_len: usize,
    new_children_len: usize,
    old: &ArenaNode<MSG>, 
    new: &ArenaNode<MSG>
) -> DataState 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    if old.tag != new.tag {
        return DataState::Tag;
    }

    if old.value != new.value {
        return DataState::Value;
    }

    if old.namespace != new.namespace {
        return DataState::Namespace;
    }

    if old.props != new.props {
        return DataState::Props;
    }

    if old_children_len != new_children_len {
        return DataState::ChildrenSize;
    }

    if old_global_index != new_global_index {
        return DataState::GlobalIndex;
    }


    DataState::None
}