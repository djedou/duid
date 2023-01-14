use indextree::{Arena, NodeId};
use crate::{
    core::{
        v_node::VirtualNode,
        duid_events::Dispatch
    },
    dom::dom_builder::DomBuilder
};
use web_sys::{
    Document, Element, Node
};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use super::patch::Patch;
use wasm_bindgen::JsCast;



pub(crate) trait ApplyPatch<DSP, MSG> 
where
    DSP: Dispatch<MSG> + Clone + 'static,
    MSG: std::fmt::Debug + Clone + 'static
{
    fn apply_patches(&self, patches: &[Patch<MSG>], new_arena: &Arena<VirtualNode<MSG>>, program: &DSP, doc: &Document, style_map: &mut HashMap<String, String>, classes_set: &mut HashSet<String>);
}

impl<DSP, MSG> ApplyPatch<DSP, MSG> for Rc<RefCell<Arena<VirtualNode<MSG>>>> 
where
    DSP: Dispatch<MSG> + Clone + 'static,
    MSG: std::fmt::Debug + Clone + 'static
{
    fn apply_patches(&self, patches: &[Patch<MSG>], new_arena: &Arena<VirtualNode<MSG>>, program: &DSP, doc: &Document, style_map: &mut HashMap<String, String>, classes_set: &mut HashSet<String>)
    {
        patches.iter().for_each(|patch| {
            match patch {
                Patch::Replace(old_id, new_id) => {
                    let new_node = new_arena.get(*new_id).expect(&format!("The {new_id:?} node does not exists")).get();
                    let new_node_to_insert = self.borrow_mut().new_node(VirtualNode {
                        tag: new_node.tag,
                        node_type: new_node.node_type.clone(),
                        namespace: new_node.namespace,
                        props: new_node.props.clone(),
                        value: new_node.value.clone(),
                        real_node: Rc::clone(&new_node.real_node),
                        active_closures: Rc::clone(&new_node.active_closures)
                    });

                    for child_id in new_id.children(&new_arena).into_iter() {
                        let child = new_arena.get(child_id).expect(&format!("The {child_id:?} node does not exists")).get();
                        
                        let child_node_to_insert = self.borrow_mut().new_node(VirtualNode {
                            tag: child.tag,
                            node_type: child.node_type.clone(),
                            namespace: child.namespace,
                            props: child.props.clone(),
                            value: child.value.clone(),
                            real_node: Rc::clone(&child.real_node),
                            active_closures: Rc::clone(&child.active_closures)
                        });
                        
                        new_node_to_insert.append(child_node_to_insert, &mut self.borrow_mut());
                        create_child(
                            &new_node_to_insert,
                            &mut self.borrow_mut(),
                            &new_arena
                        );

                    }

                    old_id.insert_before(new_node_to_insert, &mut self.borrow_mut());
                    old_id.remove_subtree(&mut self.borrow_mut());

                    self.build(program, &doc, &new_node_to_insert, style_map, classes_set);
                    let borrow = self.borrow();
                    if let Some(parent_id) = borrow[new_node_to_insert].parent() {
                        let parent = borrow.get(parent_id).expect(&format!("The {parent_id:?} node does not exists")).get();
                        let root = borrow.get(new_node_to_insert).expect(&format!("The {new_node_to_insert:?} node does not exists")).get();
                        
                        let parent_real_node = parent.real_node.borrow();
                        let root_real_node = root.real_node.borrow();
                        let _ = parent_real_node.as_ref().unwrap().append_child(root_real_node.as_ref().unwrap());
                    }

                },
                Patch::AddAttribute(old_id, attr) => {
                    let mut borrow = self.borrow_mut();
                    let old_node = borrow.get_mut(*old_id).expect(&format!("The {old_id:?} node does not exists")).get_mut();
                    old_node.props.push(attr.clone());
                    let real_node_clone = Rc::clone(&old_node.real_node);
                    
                    if let Some(root) = &*real_node_clone.borrow() {
                        let element: Element = root.to_owned().unchecked_into();
                        VirtualNode::set_element_attributes(
                            program,
                            &element,
                            &[attr],
                            &mut old_node.active_closures.borrow_mut(),
                            style_map,
                            classes_set,
                            <NodeId as Into<usize>>::into(*old_id)
                        );
                        *old_node.real_node.borrow_mut() = Some(element.unchecked_into::<Node>());
                    };
                
                },
                Patch::RemoveAttribute(old_id, attr_name) => {
                    let mut borrow = self.borrow_mut();
                    let old_node = borrow.get_mut(*old_id).expect(&format!("The {old_id:?} node does not exists")).get_mut();
                    old_node.props.retain(|prop| !(prop.name == attr_name));
                    let real_node_clone = Rc::clone(&old_node.real_node);
                    if let Some(root) = &*real_node_clone.borrow() {
                        let element: Element = root.to_owned().unchecked_into();
                        let _ = VirtualNode::<MSG>::remove_element_attribute(&element, attr_name);
                        *old_node.real_node.borrow_mut() = Some(element.unchecked_into::<Node>());
                    };
                },
                Patch::UpdateAttribute(old_id, (attr_name, new_attribute)) => {
                    let mut borrow = self.borrow_mut();
                    let old_node = borrow.get_mut(*old_id).expect(&format!("The {old_id:?} node does not exists")).get_mut();
                    old_node.props.retain(|prop| !(prop.name == *attr_name));
                    
                    let real_node_clone = Rc::clone(&old_node.real_node);
                    if let Some(root) = &*real_node_clone.borrow() {
                        let element: Element = root.to_owned().unchecked_into();
                        let _ = VirtualNode::<MSG>::remove_element_attribute(&element, attr_name);
                        VirtualNode::set_element_attribute(
                            program,
                            &element,
                            new_attribute,
                            &mut old_node.active_closures.borrow_mut(),
                            style_map,
                            classes_set,
                            <NodeId as Into<usize>>::into(*old_id)
                        );
                        *old_node.real_node.borrow_mut() = Some(element.unchecked_into::<Node>());
                    };
                },
                Patch::ChangeText(old_id, value) => {
                    if let Some(text_value) = value {
                        let mut borrow = self.borrow_mut();
                        let mut text_node = borrow.get_mut(*old_id).expect(&format!("The {old_id:?} node does not exists")).get_mut();
                        text_node.value = Some(text_value.to_owned());
                        let text_real_node = text_node.real_node.borrow();
                        let _ = text_real_node.as_ref().unwrap().set_text_content(Some(text_node.value.as_ref().unwrap()));
                    }
                }
            }
        });
    }
}


fn create_child<MSG>(
    parent_node: &NodeId,
    old_arena: &mut Arena::<VirtualNode<MSG>>,
    new_arena: &Arena::<VirtualNode<MSG>>
) 
where
    MSG: std::fmt::Debug + Clone + 'static,
{
    let new_node = new_arena.get(*parent_node).expect(&format!("The {parent_node:?} node does not exists")).get();
    let new_node_to_insert = old_arena.new_node(VirtualNode {
        tag: new_node.tag,
        node_type: new_node.node_type.clone(),
        namespace: new_node.namespace,
        props: new_node.props.clone(),
        value: new_node.value.clone(),
        real_node: Rc::clone(&new_node.real_node),
        active_closures: Rc::clone(&new_node.active_closures)
    });

    for child_id in parent_node.children(&new_arena).into_iter() {
        let child = new_arena.get(child_id).expect(&format!("The {child_id:?} node does not exists")).get();
        
        let child_node_to_insert = old_arena.new_node(VirtualNode {
            tag: child.tag,
            node_type: child.node_type.clone(),
            namespace: child.namespace,
            props: child.props.clone(),
            value: child.value.clone(),
            real_node: Rc::clone(&child.real_node),
            active_closures: Rc::clone(&child.active_closures)
        });
        
        new_node_to_insert.append(child_node_to_insert, old_arena);
        create_child(
            &new_node_to_insert,
            old_arena,
            &new_arena
        );
    }
}
