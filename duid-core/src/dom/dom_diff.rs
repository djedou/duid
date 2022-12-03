use super::patch::Patch;
use crate::core::v_node::{VirtualNode, ChangeType, PropsChangeType};
use indextree::{Arena, NodeId};
use std::rc::Rc;
use std::cell::{RefCell};


pub(crate) trait DomDiff<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    fn diff(&self, root_node_id: &NodeId, new_arena: &Arena<VirtualNode<MSG>>) -> Vec<Patch<MSG>>;
}

impl<MSG> DomDiff<MSG> for Rc<RefCell<Arena<VirtualNode<MSG>>>> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    fn diff(&self, root_node_id: &NodeId, new_arena: &Arena<VirtualNode<MSG>>) -> Vec<Patch<MSG>>
    {
        let mut patches: Vec<_> = Vec::<Patch<MSG>>::new();

        let old_arena = self.borrow();

        let old_node = old_arena.get(*root_node_id).expect(&format!("The {root_node_id:?} node does not exists on old!")).get();
        let new_node = new_arena.get(*root_node_id).expect(&format!("The {root_node_id:?} node does not exists on new!")).get();
        
        match old_node.diff(&new_node) {
            ChangeType::ChangedTag => {
                patches.push(Patch::Replace(*root_node_id, *root_node_id));
            },
            ChangeType::ChangedNodeType => {
                patches.push(Patch::Replace(*root_node_id, *root_node_id));
            },
            ChangeType::ChangedNamespace => {
                patches.push(Patch::Replace(*root_node_id, *root_node_id));
            },
            ChangeType::ChangedProps(props_changes) => {
                props_changes.iter().for_each(|prop| 
                    match prop {
                        PropsChangeType::AddedProp(attr) => {
                            patches.push(Patch::AddAttribute(*root_node_id, attr.to_owned()));
                        },
                        PropsChangeType::RemovedProp(value) => {
                            patches.push(Patch::RemoveAttribute(*root_node_id, value.to_owned()));
                        },
                        PropsChangeType::UpdatedPropValues(attr, values) => {
                            patches.push(Patch::UpdateAttribute(*root_node_id, (attr.to_owned(), values.to_owned())));
                        }
                    }
                );
            },
            ChangeType::ChangedText(value) => {
                patches.push(Patch::ChangeText(*root_node_id, value));
            },
            ChangeType::UnChanged => {
                let old_node_children = root_node_id.children(&old_arena);
                let new_node_children = root_node_id.children(&new_arena);

                if old_node_children.clone().count() != new_node_children.count() {
                    patches.push(Patch::Replace(*root_node_id, *root_node_id));
                    return patches;
                }
                else {
                    for old_child_id in old_node_children.into_iter() {
                        diff_children(&old_child_id, &old_arena, &new_arena, &mut patches);
                    }
                }
            }
        }

        patches
    }
}

fn diff_children<MSG>(node_id: &NodeId, old_arena: &Arena<VirtualNode<MSG>>, new_arena: &Arena<VirtualNode<MSG>>, patches: &mut Vec::<Patch<MSG>>) 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    let old_node = old_arena.get(*node_id).expect(&format!("The {node_id:?} node does not exists on old!")).get();
    let new_node = new_arena.get(*node_id).expect(&format!("The {node_id:?} node does not exists on new!")).get();
    
    match old_node.diff(&new_node) {
        ChangeType::ChangedTag => {
            patches.push(Patch::Replace(*node_id, *node_id));
        },
        ChangeType::ChangedNodeType => {
            patches.push(Patch::Replace(*node_id, *node_id));
        },
        ChangeType::ChangedNamespace => {
            patches.push(Patch::Replace(*node_id, *node_id));
        },
        ChangeType::ChangedProps(props_changes) => {
            props_changes.iter().for_each(|prop| 
                match prop {
                    PropsChangeType::AddedProp(attr) => {
                        patches.push(Patch::AddAttribute(*node_id, attr.to_owned()));
                    },
                    PropsChangeType::RemovedProp(value) => {
                        patches.push(Patch::RemoveAttribute(*node_id, value.to_owned()));
                    },
                    PropsChangeType::UpdatedPropValues(attr, values) => {
                        patches.push(Patch::UpdateAttribute(*node_id, (attr.to_owned(), values.to_owned())));
                    }
                }
            );
        },
        ChangeType::ChangedText(value) => {
            patches.push(Patch::ChangeText(*node_id, value.to_owned()));
        },
        ChangeType::UnChanged => {
            let old_node_children = node_id.children(&old_arena);
            let new_node_children = node_id.children(&new_arena);

            if old_node_children.clone().count() != new_node_children.count() {
                patches.push(Patch::Replace(*node_id, *node_id));
                return;
            }
            else {
                for old_child_id in old_node_children.into_iter() {
                    diff_children(&old_child_id, &old_arena, &new_arena, patches);
                }
            }
        }
    }
}