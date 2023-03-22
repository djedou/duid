use crate::arena::{Arena, ArenaNode, NodeId, ArenaNodeState, DataState, ArenaIterator, Pairs};
use std::cmp::Ordering;



pub(crate) fn patches<MSG>(
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &mut Arena<ArenaNode<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    let arena_iterator = ArenaIterator::new(old_arena.nodes_ids.clone());

    for node in &arena_iterator {
        patch_node(&node, old_arena, new_arena);
    }
    let last_node = old_arena.get_last_id();
    patch_node(&last_node, old_arena, new_arena);
    
    /*
    let old_levels: Vec<(usize, Vec<NodeId>)> = old_arena.get_nodes_ids_by_levels();
    let new_levels: Vec<(usize, Vec<NodeId>)> = new_arena.get_nodes_ids_by_levels();
    
    match old_levels.len().cmp(&new_levels.len()) {
        Ordering::Equal => { // Same size of levels or same size of deep from root node
            old_levels.iter().zip(new_levels.iter()).for_each(|(old_level, new_level)| { // for each level
                match old_level.1.len().cmp(&new_level.1.len()) { // check size of nodes for this level
                    Ordering::Equal => {
                        patch_node(&old_level.1, &new_level.1, old_arena, new_arena);
                    },
                    Ordering::Less => {
                        let mut new_level_vec = new_level.1.clone();
                        let new_level_vec_remains = new_level_vec.split_off(old_level.1.len());
                        patch_node(&old_level.1, &new_level_vec, old_arena, new_arena);
                        new_level_vec_remains.iter().for_each(|id| {
                            mark_inserted_state(id.clone(), old_arena, &new_arena);
                            mark_children_state::<MSG>(&ArenaNodeState::InsertedChild, &[id.clone()], old_arena, new_arena);
                        });
                    },
                    Ordering::Greater => {
                        let mut old_level_vec = old_level.1.clone();
                        let old_level_vec_remains = old_level_vec.split_off(new_level.1.len());
                        patch_node(&old_level_vec, &new_level.1, old_arena, new_arena);
                        old_level_vec_remains.iter().for_each(|id| {
                            mark_parent_removed_state(id.clone(), old_arena);
                            mark_children_removed_state::<MSG>(&[id.clone()], old_arena);
                        });
                    }
                }
            })
        },
        Ordering::Less => {
            let mut new_levels_clone = new_levels.clone();
            let new_levels_clone_vec = new_levels_clone.split_off(old_levels.len());

            old_levels.iter().zip(new_levels_clone.iter()).for_each(|(old_level, new_level)| {
                match old_level.1.len().cmp(&new_level.1.len()) {
                    Ordering::Equal => {
                        patch_node(&old_level.1, &new_level.1, old_arena, new_arena);
                    },
                    Ordering::Less => {
                        let mut new_level_vec = new_level.1.clone();
                        let new_level_vec_remains = new_level_vec.split_off(old_level.1.len());
                        patch_node(&old_level.1, &new_level_vec, old_arena, new_arena);
                        new_level_vec_remains.iter().for_each(|id| {
                            mark_inserted_state(id.clone(), old_arena, &new_arena);
                            mark_children_state::<MSG>(&ArenaNodeState::InsertedChild, &[id.clone()], old_arena, new_arena);
                        });
                    },
                    Ordering::Greater => {
                        let mut old_level_vec = old_level.1.clone();
                        let old_level_vec_remains = old_level_vec.split_off(new_level.1.len());
                        patch_node(&old_level_vec, &new_level.1, old_arena, new_arena);
                        old_level_vec_remains.iter().for_each(|id| {
                            mark_parent_removed_state(id.clone(), old_arena);
                            mark_children_removed_state::<MSG>(&[id.clone()], old_arena);
                        });
                    }
                }
            });

            new_levels_clone_vec.iter().for_each(|(level, new_level_ids)| {
                new_level_ids.iter().for_each(|id| {
                    mark_inserted_state(id.clone(), old_arena, &new_arena);
                    mark_children_state::<MSG>(&ArenaNodeState::InsertedChild, &[id.clone()], old_arena, new_arena);
                });
            });
        },
        Ordering::Greater => {
            let mut old_levels_clone = old_levels.clone();
            let old_levels_clone_vec = old_levels_clone.split_off(new_levels.len());

            old_levels_clone.iter().zip(new_levels.iter()).for_each(|(old_level, new_level)| {
                match old_level.1.len().cmp(&new_level.1.len()) {
                    Ordering::Equal => {
                        patch_node(&old_level.1, &new_level.1, old_arena, new_arena);
                    },
                    Ordering::Less => {
                        let mut new_level_vec = new_level.1.clone();
                        let new_level_vec_remains = new_level_vec.split_off(old_level.1.len());
                        patch_node(&old_level.1, &new_level_vec, old_arena, new_arena);
                        new_level_vec_remains.iter().for_each(|id| {
                            mark_inserted_state(id.clone(), old_arena, &new_arena);
                            mark_children_state::<MSG>(&ArenaNodeState::InsertedChild, &[id.clone()], old_arena, new_arena);
                        });
                    },
                    Ordering::Greater => {
                        let mut old_level_vec = old_level.1.clone();
                        let old_level_vec_remains = old_level_vec.split_off(new_level.1.len());
                        patch_node(&old_level_vec, &new_level.1, old_arena, new_arena);
                        old_level_vec_remains.iter().for_each(|id| {
                            mark_parent_removed_state(id.clone(), old_arena);
                            mark_children_removed_state::<MSG>(&[id.clone()], old_arena);
                        });
                    }
                }
            });

            old_levels_clone_vec.iter().for_each(|(level, old_level_ids)| {
                old_level_ids.iter().for_each(|id| {
                    mark_parent_removed_state(id.clone(), old_arena);
                    mark_children_removed_state::<MSG>(&[id.clone()], old_arena);
                });
            });
        }
    }*/
}


fn patch_node<MSG>(
    old_id: &NodeId,
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &mut Arena<ArenaNode<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    let old_binding = new_arena.nodes_ids.clone();
    let mut finded_new_id: Vec<_> = old_binding.iter().collect();
    
    match finded_new_id.iter_mut()
                .find(|new| 
                    new.child.level == old_id.level && 
                    new.child.level_index == old_id.level_index) 
        {
            Some(new_id) => {
                // get old_node
                // get new_node
                // get old_node children
                // get new_node children
                match *old_id == new_id.child {
                    true => {
                        match (old_id.get_node_by_id_mut(old_arena), new_id.child.get_node_by_id_mut(new_arena)) {
                            (Some(old_node), Some(new_node)) => {
                                let node_state = old_node.node_state.clone();
                                match old_node == new_node {
                                    true => {
                                        match old_id.get_children(&old_binding).len() == new_id.child.get_children(&new_arena.nodes_ids).len() {
                                            true => {
                                                old_node.node_state = ArenaNodeState::UnChanged;
                                            },
                                            false => {
                                                match &node_state {
                                                    ArenaNodeState::None => {
                                                        old_node.node_state = ArenaNodeState::Removed;
                                                        old_arena.removed_ids.insert(old_id.clone());
                                                        mark_children_removed_state::<MSG>(&[old_id.clone()], old_arena);
                                                        mark_replacing_state(new_id.child.clone(), old_id.clone(), old_arena, new_arena);
                                                        mark_children_state::<MSG>(&ArenaNodeState::Visited, &[new_id.child.clone()], old_arena, new_arena);
                                                    },
                                                    _ => {}
                                                }
                                            }
                                        }
                                    },
                                    false => {
                                        match &node_state {
                                            ArenaNodeState::None => {
                                                old_node.node_state = ArenaNodeState::Removed;
                                                old_arena.removed_ids.insert(old_id.clone());
                                                mark_children_removed_state::<MSG>(&[old_id.clone()], old_arena);
                                                mark_replacing_state(new_id.child.clone(), old_id.clone(), old_arena, new_arena);
                                                mark_children_state::<MSG>(&ArenaNodeState::Visited, &[new_id.child.clone()], old_arena, new_arena);
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    },
                    false => { // global index are differents
                        match (old_id.get_node_by_id_mut(old_arena), new_id.child.get_node_by_id_mut(new_arena)) {
                            (Some(old_node), Some(new_node)) => {
                                let node_state = old_node.node_state.clone();
                                match &node_state {
                                    ArenaNodeState::None => {
                                        old_node.node_state = ArenaNodeState::Removed;
                                        old_arena.removed_ids.insert(old_id.clone());
                                        mark_children_removed_state::<MSG>(&[old_id.clone()], old_arena);
                                        mark_replacing_state(new_id.child.clone(), old_id.clone(), old_arena, new_arena);
                                        mark_children_state::<MSG>(&ArenaNodeState::Visited, &[new_id.child.clone()], old_arena, new_arena);
                                    },
                                    _ => {}
                                }

                            },
                            _ => {}
                        }
                    }
                }
            },
            None => {} // we should not reach here
        }

    /*
    old_level.iter().zip(new_level.iter()).for_each(|(old_id, new_id)| {
        match (old_id.get_node_by_id_mut(old_arena), new_id.get_node_by_id_mut(new_arena)) {
            (Some(old_node), Some(new_node)) => {
                // check if already passed
                match (&old_node.node_state, &new_node.node_state) {
                    (&ArenaNodeState::None, &ArenaNodeState::None) => {
                        match (old_node == new_node, old_id == new_id) {
                            (true, true) => {
                                old_node.node_state = ArenaNodeState::UnChanged;
                            },
                            (true, false) => {
                                // we need to update duid-id to the new_id in html node
                                old_node.node_state = ArenaNodeState::IdChanged(old_id.clone(), new_id.clone());
                                old_node.id = new_id.clone();
                                if let Some(pair) = old_id.get_pair_mut(&mut old_arena.node_id_pairs) {
                                    pair[1] = new_id.clone();
                                    if let Some(parent) = new_id.get_parent(&new_arena.node_id_pairs) {
                                        if !old_arena.new_node_id_pairs.contains(&[parent.clone(), new_id.clone()]) {
                                            old_arena.new_node_id_pairs.push([parent.clone(), new_id.clone()]);
                                        }
                                    }
                                }
                            },
                            (false, _) => {
                                old_node.node_state = ArenaNodeState::Removed;
                                old_arena.removed_ids.push(old_id.clone());
                                mark_children_removed_state::<MSG>(&[old_id.clone()], old_arena);
                                mark_replacing_state(new_id.clone(), old_id.clone(), old_arena, new_arena);
                                mark_children_state::<MSG>(&ArenaNodeState::ReplacingChild, &[new_id.clone()], old_arena, new_arena);
                            }
                        }
                    },
                    _ => {
                        // already passed, so nothing to do here
                    }
                }
            },
            _ => {
                // we should not reach here
            }
        }
    });*/
}

fn mark_inserted_state<MSG>(
    node: NodeId, 
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &Arena<ArenaNode<MSG>>
) 
where 
    MSG: Clone
{
    /*
        match node.get_parent(&new_arena.node_id_pairs) {
            Some(parent) => {
                match node.get_node_by_id(&new_arena) {
                    Some(child_node) => {
                        let mut new_child_node = child_node.clone();
                        new_child_node.node_state = ArenaNodeState::Inserted(parent.clone());
                        old_arena.nodes.push(new_child_node);
                        if !old_arena.new_node_id_pairs.contains(&[parent.clone(), node.clone()]) {
                            old_arena.new_node_id_pairs.push([parent.clone(), node.clone()]);
                        }
                    },
                    None => {}
                }
            },  
            None => {}
        }*/
}

fn mark_parent_removed_state<MSG>(
    node: NodeId, 
    old_arena: &mut Arena<ArenaNode<MSG>>
) 
where 
    MSG: Clone
{
    /*
    match node.get_node_by_id_mut(old_arena) {
        Some(child_node) => {
            if child_node.node_state == ArenaNodeState::None {
                child_node.node_state = ArenaNodeState::Removed;
                old_arena.removed_ids.push(node.clone());
            }
        },
        None => {}
    }*/
}

fn mark_replacing_state<MSG>(
    node: NodeId,
    old_id: NodeId,
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &mut Arena<ArenaNode<MSG>>
) 
where 
    MSG: Clone
{
        match node.get_parent(&new_arena.nodes_ids) {
            Some(parent) => {
                match node.get_node_by_id_mut(new_arena) {
                    Some(child_node) => {
                        let mut new_child_node = child_node.clone();
                        new_child_node.id = node.clone();
                        new_child_node.node_state = ArenaNodeState::Replacing(old_id.clone());
                        old_arena.nodes.push(new_child_node);
                        old_arena.new_node_id_pairs.insert(Pairs::new(parent.clone(), node.clone()));
                    },
                    None => {}
                }
            },
            None => {}
        }
}

fn mark_children_removed_state<MSG>(parents_nodes: &[NodeId], old_arena: &mut Arena<ArenaNode<MSG>>) 
where 
    MSG: Clone
{
    parents_nodes.iter().for_each(|parent| {
        let children = parent.get_children(&old_arena.nodes_ids);
        children.iter().for_each(|child| {
            match child.get_node_by_id_mut(old_arena) {
                Some(child_node) => {
                    if child_node.node_state == ArenaNodeState::None {
                        child_node.node_state = ArenaNodeState::Removed;
                        old_arena.removed_ids.insert(child.clone());
                    }
                },
                None => {}
            }
        });

        mark_children_removed_state::<MSG>(&children, old_arena);
    });
}

fn mark_children_state<MSG>(state: &ArenaNodeState, parents_nodes: &[NodeId], old_arena: &mut Arena<ArenaNode<MSG>>, new_arena: &mut Arena<ArenaNode<MSG>>) 
where 
    MSG: Clone
{
    /*
    parents_nodes.iter().for_each(|parent| {
        let children = parent.get_children(&new_arena.node_id_pairs);
        children.iter().for_each(|child| {
            match child.get_node_by_id_mut(new_arena) {
                Some(child_node) => {
                    child_node.node_state = state.clone();
                    old_arena.nodes.push(child_node.clone());
                    if !old_arena.new_node_id_pairs.contains(&[parent.clone(), child.clone()]) {
                        old_arena.new_node_id_pairs.push([parent.clone(), child.clone()]);
                    }
                },
                None => {}
            }
        });
        
        mark_children_state::<MSG>(&state, &children, old_arena, new_arena);
    });
    */
}
/*
fn get_data_changed<MSG>(old: &ArenaNode<MSG>, new: &ArenaNode<MSG>) -> DataState 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    if old.value != new.value {
        return DataState::Value;
    }

    if old.tag != new.tag {
        return DataState::Tag;
    }

    if old.node_type != new.node_type {
        return DataState::NodeType;
    }

    if old.namespace != new.namespace {
        return DataState::Namespace;
    }

    if old.props != new.props {
        return DataState::Props;
    }

    DataState::None
}
*/