use crate::{
    v_dom::{Vdom, Vnode, CreatedNode},
    event_manager::Dispatcher
};
use wasm_bindgen::JsCast;
use std::fmt::Debug;
use super::document;
use indextree::{NodeId};
use std::rc::Rc;
use std::cell::RefCell;

use web_sys::{
    self, Element, Node
};



#[derive(Debug, Clone)]
pub struct RDom {
    pub mount_node: Node,
    pub replace: bool,
    pub use_shadow: bool,
}


impl RDom {
    pub(crate) fn new(
        mount: &str,
        replace: bool,
        use_shadow: bool,
    ) -> RDom {
        
        let node = 
            document()
            .get_element_by_id(mount)
            .expect(&format!("element with id {:?} not present", mount))
            .unchecked_into::<Node>();
        
        RDom {
            mount_node: node,
            replace,
            use_shadow,
        }
    }
}

impl RDom {

    pub fn mount(&mut self, duid: Rc<RefCell<Dispatcher>>, current_v_node: &Vdom<Vnode>, app_node_id: &NodeId) {
        
        let created_node = CreatedNode::create_dom_node(duid.clone(), current_v_node, &app_node_id);
        
        if self.replace {
            let mount_element: &Element = self.mount_node.unchecked_ref();
            mount_element
                .replace_with_with_node_1(&created_node.node)
                .expect("Could not append child to mount");
        } else {
            if self.use_shadow {
                let mount_element: &web_sys::Element =
                    self.mount_node.unchecked_ref();
                mount_element
                    .attach_shadow(&web_sys::ShadowRootInit::new(
                        web_sys::ShadowRootMode::Open,
                    ))
                    .expect("unable to attached shadow");
                let mount_shadow =
                    mount_element.shadow_root().expect("must have a shadow");

                let mount_shadow_node: &web_sys::Node =
                    mount_shadow.unchecked_ref();

                mount_shadow_node
                    .append_child(&created_node.node)
                    .expect("could not append child to mount shadow");
            } else {
                self.mount_node
                    .append_child(&created_node.node)
                    .expect("Could not append child to mount");
            }
        }
        
        //self.root_node = Some(created_node.node);
        //self.active_closures = created_node.closures;
        //self.set_focus_element();
        
    }

    /*
    fn set_focus_element(&self) {
        if let Some(focused_node) = &self.focused_node {
            let focused_element: &Element = focused_node.unchecked_ref();
            CreatedNode::set_element_focus(focused_element);
        }
    }
    */
    pub fn _mount_node(&self) -> Node {
        self.mount_node.clone()
    }
}
