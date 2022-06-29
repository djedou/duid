use crate::{
    event_manager::Dispatch,

    v_dom::{ActiveClosure, CreatedNode},
    v_dom::v_node
};
use wasm_bindgen::JsCast;
use web_sys::{self, Element, Node};
use std::fmt::Debug;
use super::document;



#[derive(Debug)]
pub struct RDom {
    pub root_node: Option<Node>,
    pub mount_node: Node,
    pub active_closures: ActiveClosure,
    pub focused_node: Option<Node>,
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
            root_node: None,
            mount_node: node,
            active_closures: ActiveClosure::new(),
            focused_node: None,
            replace,
            use_shadow,
        }
    }
}

impl RDom {

    pub fn mount<DSP, MSG>(&mut self, program: &DSP, current_v_node: &v_node::Node<MSG>)
    where
        MSG: 'static + Debug,   
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let created_node = CreatedNode::create_dom_node(
            program,
            current_v_node,
            &mut self.focused_node,
        );

        //TODO: maybe remove replace the mount
        if self.replace {
            tracing::info!("djed");
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
        self.root_node = Some(created_node.node);
        self.active_closures = created_node.closures;
        self.set_focus_element();
    }


    fn set_focus_element(&self) {
        if let Some(focused_node) = &self.focused_node {
            let focused_element: &Element = focused_node.unchecked_ref();
            CreatedNode::set_element_focus(focused_element);
        }
    }


    pub fn _root_node(&self) -> Node {
        self.root_node
            .as_ref()
            .expect("must have a root_node")
            .clone()
    }


    pub fn _mount_node(&self) -> Node {
        self.mount_node.clone()
    }
}