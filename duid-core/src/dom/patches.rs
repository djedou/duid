use crate::arena::{Arena, ArenaNode, NodeId, ArenaNodeState};
use std::cmp::Ordering;



pub(crate) fn patches<MSG>(old_arena: &mut Arena<ArenaNode<MSG>>, new_arena: &Arena<ArenaNode<MSG>>) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    
    let old_levels: Vec<(usize, Vec<NodeId>)> = old_arena.get_nodes_ids_by_levels();
    let new_levels: Vec<(usize, Vec<NodeId>)> = new_arena.get_nodes_ids_by_levels();

    match old_levels.len().cmp(&new_levels.len()) {
        Ordering::Equal => {
            old_levels.iter().zip(new_levels.iter()).for_each(|(old_level, new_level)| {
                match old_level.1.len().cmp(&new_level.1.len()) {
                    Ordering::Equal => {
                        patch_node(&old_level.1, &new_level.1, old_arena, &new_arena);
                    },
                    Ordering::Less => {
                        let mut new_level_vec = new_level.1.clone();
                        let new_level_vec_remains = new_level_vec.split_off(old_level.1.len());
                        patch_node(&old_level.1, &new_level_vec, old_arena, &new_arena);
                        new_level_vec_remains.iter().for_each(|id| {
                            mark_children_added_state(&[id.clone()], old_arena, &new_arena);
                            mark_inserted_state(id.clone(), old_arena, &new_arena);
                        });
                    },
                    Ordering::Greater => {
                        let mut old_level_vec = old_level.1.clone();
                        let old_level_vec_remains = old_level_vec.split_off(new_level.1.len());
                        patch_node(&old_level_vec, &new_level.1, old_arena, &new_arena);
                        old_level_vec_remains.iter().for_each(|id| {
                            mark_children_remoded_state::<MSG>(&[id.clone()], old_arena);
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
                        patch_node(&old_level.1, &new_level.1, old_arena, &new_arena);
                    },
                    Ordering::Less => {
                        let mut new_level_vec = new_level.1.clone();
                        let new_level_vec_remains = new_level_vec.split_off(old_level.1.len());
                        patch_node(&old_level.1, &new_level_vec, old_arena, &new_arena);
                        new_level_vec_remains.iter().for_each(|id| {
                            mark_children_added_state(&[id.clone()], old_arena, &new_arena);
                            mark_inserted_state(id.clone(), old_arena, &new_arena);
                        });
                    },
                    Ordering::Greater => {
                        let mut old_level_vec = old_level.1.clone();
                        let old_level_vec_remains = old_level_vec.split_off(new_level.1.len());
                        patch_node(&old_level_vec, &new_level.1, old_arena, &new_arena);
                        old_level_vec_remains.iter().for_each(|id| {
                            mark_children_remoded_state::<MSG>(&[id.clone()], old_arena);
                        });
                    }
                }
            });
            
            new_levels_clone_vec.iter().for_each(|(level, new_level_ids)| {
                new_level_ids.iter().for_each(|id| {
                    mark_children_added_state(&[id.clone()], old_arena, &new_arena);
                    mark_inserted_state(id.clone(), old_arena, &new_arena);
                });
            });

        },
        Ordering::Greater => {
            let mut old_levels_clone = old_levels.clone();
            let old_levels_clone_vec = old_levels_clone.split_off(new_levels.len());
            
            old_levels_clone.iter().zip(new_levels.iter()).for_each(|(old_level, new_level)| {
                match old_level.1.len().cmp(&new_level.1.len()) {
                    Ordering::Equal => {
                        patch_node(&old_level.1, &new_level.1, old_arena, &new_arena);
                    },
                    Ordering::Less => {
                        let mut new_level_vec = new_level.1.clone();
                        let new_level_vec_remains = new_level_vec.split_off(old_level.1.len());
                        patch_node(&old_level.1, &new_level_vec, old_arena, &new_arena);
                        new_level_vec_remains.iter().for_each(|id| {
                            mark_children_added_state(&[id.clone()], old_arena, &new_arena);
                            mark_inserted_state(id.clone(), old_arena, &new_arena);
                        });
                    },
                    Ordering::Greater => {
                        let mut old_level_vec = old_level.1.clone();
                        let old_level_vec_remains = old_level_vec.split_off(new_level.1.len());
                        patch_node(&old_level_vec, &new_level.1, old_arena, &new_arena);
                        old_level_vec_remains.iter().for_each(|id| {
                            mark_children_remoded_state::<MSG>(&[id.clone()], old_arena);
                        });
                    }
                }
            });

            old_levels_clone_vec.iter().for_each(|(level, old_level_ids)| {
                old_level_ids.iter().for_each(|id| {
                    mark_children_remoded_state::<MSG>(&[id.clone()], old_arena);
                });
            });
        }
    }
}

fn patch_node<MSG>(
    old_level: &[NodeId], 
    new_level: &[NodeId],
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &Arena<ArenaNode<MSG>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    old_level.iter().zip(new_level.iter()).for_each(|(old_id, new_id)| {
        match (old_id.get_node_by_id_mut(old_arena), new_id.get_node_by_id(&new_arena)) {
            (Some(old_node), Some(new_node)) => {
                // check if already passed
                match (&old_node.node_state, &new_node.node_state) {
                    (&ArenaNodeState::None, &ArenaNodeState::None) => {
                        match (old_node == new_node, old_id == new_id) {
                            (true, true) => {
                                old_node.node_state = ArenaNodeState::UnChanged;
                            },
                            (true, false) => {
                                old_node.node_state = ArenaNodeState::IdChanged(new_id.clone());
                            },
                            (false, true) => {
                                old_node.node_state = ArenaNodeState::ReplaceBy(new_id.clone());
                                mark_children_remoded_state::<MSG>(&[old_id.clone()], old_arena);
                                mark_children_added_state(&[new_id.clone()], old_arena, &new_arena);
                                mark_parent_replace_state(new_id.clone(), old_arena, &new_arena);
                            },
                            (false, false) => {
                                old_node.node_state = ArenaNodeState::ReplaceBy(new_id.clone());
                                mark_children_remoded_state::<MSG>(&[old_id.clone()], old_arena);
                                mark_children_added_state(&[new_id.clone()], old_arena, &new_arena);
                                mark_parent_replace_state(new_id.clone(), old_arena, &new_arena);
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
    });
}

fn mark_inserted_state<MSG>(
    node: NodeId, 
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &Arena<ArenaNode<MSG>>
) 
where 
    MSG: Clone
{
        match node.get_parent(&new_arena.node_id_pairs) {
            Some(parent) => {
                match node.get_node_by_id(&new_arena) {
                    Some(child_node) => {
                        let mut new_child_node = child_node.clone();
                        new_child_node.node_state = ArenaNodeState::ChildInserted;
                        old_arena.nodes.push(new_child_node);
                        old_arena.node_id_pairs.push([parent.clone(), node.clone()]);
                    },
                    None => {}
                }
            },
            None => {}
        }
}

fn mark_parent_replace_state<MSG>(
    node: NodeId, 
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &Arena<ArenaNode<MSG>>
) 
where 
    MSG: Clone
{
        match node.get_parent(&new_arena.node_id_pairs) {
            Some(parent) => {
                match node.get_node_by_id(&new_arena) {
                    Some(child_node) => {
                        let mut new_child_node = child_node.clone();
                        new_child_node.node_state = ArenaNodeState::Added;
                        old_arena.nodes.push(new_child_node);
                        old_arena.node_id_pairs.push([parent.clone(), node.clone()]);
                    },
                    None => {}
                }
            },
            None => {}
        }
}

fn mark_children_added_state<MSG>(
    parents_nodes: &[NodeId], 
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &Arena<ArenaNode<MSG>>
) 
where 
    MSG: Clone
{
    parents_nodes.iter().for_each(|parent| {
        let children = parent.get_children(&new_arena.node_id_pairs);
        children.iter().for_each(|child| {
            match child.get_node_by_id(&new_arena) {
                Some(child_node) => {
                    let mut new_child_node = child_node.clone();
                    new_child_node.node_state = ArenaNodeState::Added;
                    old_arena.nodes.push(new_child_node);
                    old_arena.node_id_pairs.push([parent.clone(), child.clone()]);
                },
                None => {}
            }
        });

        mark_children_added_state::<MSG>(&children, old_arena, new_arena);
    });
}

fn mark_children_remoded_state<MSG>(parents_nodes: &[NodeId], old_arena: &mut Arena<ArenaNode<MSG>>) 
where 
    MSG: Clone
{
    parents_nodes.iter().for_each(|parent| {
        let children = parent.get_children(&old_arena.node_id_pairs);
        children.iter().for_each(|child| {
            match child.get_node_by_id_mut(old_arena) {
                Some(child_node) => {
                    child_node.node_state = ArenaNodeState::Removed;
                },
                None => {}
            }
        });

        mark_children_remoded_state::<MSG>(&children, old_arena);
    });
}