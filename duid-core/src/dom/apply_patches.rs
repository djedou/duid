use crate::{
    core::{
        v_node::{VirtualNode, ChangeType, PropsChangeType, VirtualNodeType},
        duid_events::Dispatch
    }
};
use web_sys::{
    Document, Element, Node
};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use wasm_bindgen::JsCast;



pub(crate) trait ApplyPatch<DSP, MSG> 
where
    DSP: Dispatch<MSG> + Clone + 'static,
    MSG: std::fmt::Debug + Clone + 'static
{
    fn apply_patches(&mut self, new_node: &VirtualNode<MSG>, program: &DSP, doc: &Document, style_map: &mut HashMap<String, String>, selectors_set: &mut HashSet<String>);
}

impl<DSP, MSG> ApplyPatch<DSP, MSG> for VirtualNode<MSG> 
where
    DSP: Dispatch<MSG> + Clone + 'static,
    MSG: std::fmt::Debug + Clone + 'static
{
    fn apply_patches(&mut self, new_node: &VirtualNode<MSG>, program: &DSP, doc: &Document, style_map: &mut HashMap<String, String>, selectors_set: &mut HashSet<String>)
    {
        crate::console::info!("tag: {:#?}", self.tag);
        crate::console::info!("node_type: {:#?}", self.node_type);
        match self.diff(&new_node) {
            ChangeType::ChangedTag => {
                new_node.build_node(program, &doc, style_map, selectors_set);
                self.replace_by(new_node);
            },
            ChangeType::ChangedNodeType => {
                new_node.build_node(program, &doc, style_map, selectors_set);
                self.replace_by(new_node);
            },
            ChangeType::ChangedNamespace => {
                new_node.build_node(program, &doc, style_map, selectors_set);
                self.replace_by(new_node);
            },
            ChangeType::ChangedProps(props_changes) => {
                props_changes.iter().for_each(|prop| 
                    match prop {
                        PropsChangeType::AddedProp(attr) => {
                            self.props.push(attr.clone());
                            let real_node_clone = Rc::clone(&self.real_node);
                            let mut new_element = None;
                            
                            if let Some(root) = &*real_node_clone.borrow() {
                                let element: Element = root.to_owned().unchecked_into();
                                VirtualNode::set_element_attributes(
                                    program,
                                    &element,
                                    &[attr],
                                    &mut self.active_closures.borrow_mut(),
                                    style_map,
                                    selectors_set
                                );
                                new_element = Some(element.unchecked_into::<Node>());
                            };
        
                            if let Some(_) = &new_element {
                                *self.real_node.borrow_mut() = new_element;
                            }
                        },
                        PropsChangeType::RemovedProp(value) => {
                            let attr_name = value.to_owned();
                            self.props.retain(|prop| !(prop.name == attr_name));
                            let real_node_clone = Rc::clone(&self.real_node);
                            let mut new_element = None;

                            if let Some(root) = &*real_node_clone.borrow() {
                                let element: Element = root.to_owned().unchecked_into();
                                let _ = VirtualNode::<MSG>::remove_element_attribute(&element, &attr_name);
                                new_element = Some(element.unchecked_into::<Node>());
                            };
                            
                            if let Some(_) = &new_element {
                                *self.real_node.borrow_mut() = new_element;
                            }
                        },
                        PropsChangeType::UpdatedPropValues(attr, values) => {
                            let attr_name = attr.to_owned();
                            let new_attribute = values.to_owned();
                            
                            self.props.retain(|prop| !(prop.name == attr_name));
                            let real_node_clone = Rc::clone(&self.real_node);
                            let mut new_element = None;

                            if let Some(root) = &*real_node_clone.borrow() {
                                let element: Element = root.to_owned().unchecked_into();
                                let _ = VirtualNode::<MSG>::remove_element_attribute(&element, &attr_name);
                                VirtualNode::set_element_attribute(
                                    program,
                                    &element,
                                    &new_attribute,
                                    &mut self.active_closures.borrow_mut(),
                                    style_map,
                                    selectors_set
                                );
                                new_element = Some(element.unchecked_into::<Node>());
                            };

                            if let Some(_) = &new_element {
                                *self.real_node.borrow_mut() = new_element;
                            }
                        }
                    }
                );
            },
            ChangeType::ChangedText(value) => {
                if let Some(text_value) = value {
                    let text_real_node = self.real_node.borrow();
                    let _ = text_real_node.as_ref().unwrap().set_text_content(Some(&text_value));
                    self.value = Some(text_value.to_owned());
                }
            },
            ChangeType::UnChanged => {
                let old_node_children_size = self.children.len();
                let new_node_children_size = new_node.children.len();

                match old_node_children_size >= new_node_children_size {
                    true => {
                        for i in 0..old_node_children_size {
                            match (self.children.get_mut(i), new_node.children.get(i)) {
                                (Some(old_child), Some(new_child)) => {
                                    old_child.apply_patches(new_child, program, &doc, style_map, selectors_set);
                                },
                                (Some(old_child), None) => {
                                    if let Some(old_node) = &*old_child.real_node.borrow() {
                                        let old_element: &Element = old_node.unchecked_ref();
                                        old_element.remove();
                                    }

                                    let _ = self.children.remove(i);
                                },
                                _ => {}
                            }
                        }
                    },
                    false => {
                        for i in 0..new_node_children_size {
                            match (self.children.get_mut(i), new_node.children.get(i)) {
                                (Some(old_child), Some(new_child)) => {
                                    old_child.apply_patches(new_child, program, &doc, style_map, selectors_set);
                                },
                                (None, Some(new_child)) => {
                                    if let Some(real_node_parent) = &*self.real_node.borrow() {
                                        if let Some(real_node_child) = &*new_child.real_node.borrow() {
                                            let _ = real_node_parent.append_child(&real_node_child);
                                        }
                                    }

                                    self.children.push(new_child.to_owned());
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}