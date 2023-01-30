use indextree::{Arena, NodeId};
use crate::core::{
    v_node::VirtualNode,
    duid_events::Dispatch
};
use web_sys::{
    Document
};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;


pub trait DomBuilder<DSP, MSG> 
where
    DSP: Dispatch<MSG> + Clone + 'static,
    MSG: std::fmt::Debug + Clone + 'static
{
    fn build(&self, program: &DSP, doc: &Document, node_id: &NodeId, style_map: &mut HashMap<String, String>, selectors_set: &mut HashSet<String>);
}

impl<DSP, MSG> DomBuilder<DSP, MSG> for Rc<RefCell<Arena<VirtualNode<MSG>>>> 
where 
    DSP: Dispatch<MSG> + Clone + 'static,
    MSG: std::fmt::Debug + Clone + 'static
{
    fn build(&self, program: &DSP, doc: &Document, root_node_id: &NodeId, style_map: &mut HashMap<String, String>, selectors_set: &mut HashSet<String>)
    {
        // build each real node
        for node_id in root_node_id.descendants(&self.borrow()).into_iter() {
            let node_borrow = self.borrow();
            let node = node_borrow.get(node_id).expect(&format!("The {node_id:?} node does not exists")).get();
            node.build_node(program, &doc, style_map, selectors_set, <NodeId as Into<usize>>::into(node_id));
        }

        // build each real node's children
        for node_id in root_node_id.descendants(&self.borrow()).into_iter() {
            let node_borrow = self.borrow();
            let node = node_borrow.get(node_id).expect(&format!("The {node_id:?} node does not exists")).get();
            
            for child_id in node_id.children(&self.borrow()).into_iter() {
                let child = node_borrow.get(child_id).expect(&format!("The {node_id:?} node does not exists")).get();
                if let Some(real_node_parent) = &*node.real_node.borrow() {
                    if let Some(real_node_child) = &*child.real_node.borrow() {
                        let _ = real_node_parent.append_child(&real_node_child);
                    }
                }
            }
        }
    }
}