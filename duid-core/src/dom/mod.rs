mod dom_builder;
mod patch;
mod dom_diff;
mod apply_patches;

use std::rc::Rc;
use std::cell::RefCell;
use crate::core::duid_events::Dispatch;
use wasm_bindgen::JsCast;
use std::fmt::Debug;
use web_sys::{
    self, Element, Node, Document
};
use indextree::{Arena, NodeId};
use crate::core::{
    v_node::VirtualNode,
    util::document
};
use std::collections::{HashMap, HashSet};
pub use dom_builder::*;
pub(crate) use dom_diff::DomDiff;
pub(crate) use apply_patches::ApplyPatch;




#[derive(Debug, Clone)]
pub(crate) struct Dom<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    pub(crate) mount_node: Node,
    pub(crate) replace: bool,
    pub(crate) use_shadow: bool,
    pub(crate) arena: Rc<RefCell<Arena<VirtualNode<MSG>>>>,
    pub(crate) arena_root_node_id: Option<NodeId>,
    pub(crate) document: Document,
    pub(crate) base_styles: HashMap<String, String>,
    pub(crate) styles: HashMap<String, String>
}


impl<MSG> Dom<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    pub(crate) fn new<DSP>(
        mount: &str,
        replace: bool,
        use_shadow: bool,
        base_styles: HashMap<String, String>,
        styles: HashMap<String, String>
    ) -> Dom<MSG> 
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let doc = document();
        let node = 
            doc
            .get_element_by_id(mount)
            .expect(&format!("element with id {:?} not present", mount))
            .unchecked_into::<Node>();
        
        Dom::<MSG> {
            mount_node: node,
            replace,
            use_shadow, 
            arena: Rc::new(RefCell::new(Arena::new())),
            arena_root_node_id: None,
            document: doc,
            base_styles,
            styles
        }
    }
}

impl<MSG> Dom<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    pub(crate) fn mount<DSP>(&mut self, program: &DSP, arena: Arena<VirtualNode<MSG>>, root_node_id: NodeId) 
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let mut style_map: HashMap<String, String> = HashMap::with_capacity(0); 
        let mut classes_map: HashSet<String> = HashSet::with_capacity(0); 
        self.arena_root_node_id = Some(root_node_id);
        *self.arena.borrow_mut() = arena;
        self.arena.build(program, &self.document, &self.arena_root_node_id.as_ref().unwrap(), &mut style_map, &mut classes_map);
        
        self.first_mount(&self.mount_styles(style_map, true));
    }

    pub(crate) fn render<DSP>(&mut self, program: &DSP, arena: Arena<VirtualNode<MSG>>, root_node_id: NodeId) 
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let mut style_map: HashMap<String, String> = HashMap::with_capacity(0);
        let mut classes_map: HashSet<String> = HashSet::with_capacity(0); 
        let patches: Vec<_> = self.arena.diff(&root_node_id, &arena);
        self.arena.apply_patches(&patches, &arena, program, &self.document, &mut style_map, &mut classes_map);
        self.inject_styles(&self.mount_styles(style_map, false));
    }

    fn first_mount(&self, styles: &[(String, String)]) {
        let node_borrow = self.arena.borrow();
        let virtual_node = node_borrow.get(*self.arena_root_node_id.as_ref().unwrap()).expect("The node does not exists").get();

        if let Some(node) = &*virtual_node.real_node.borrow() {
            if self.replace {
                let mount_element: &Element = self.mount_node.unchecked_ref();
                mount_element
                    .replace_with_with_node_1(node)
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
                        .append_child(&node)
                        .expect("could not append child to mount shadow");
                } else {
                    self.mount_node
                        .append_child(&node)
                        .expect("Could not append child to mount");
                }
            }
        };


        self.inject_styles(styles);
    }

    fn inject_styles(&self, styles: &[(String, String)]) {
        styles.iter().for_each(|(name, style)| self.inject_style(name, style));
    }

    fn inject_style(&self, name: &str, style: &str) {

        if let Some(style_to_update) = self.document.get_element_by_id(name) {
            let html_style: web_sys::Node = style_to_update.unchecked_into();
            html_style.set_text_content(Some(style));
        } else {
            let html_style = self.document
                .create_element("style")
                .expect("must be able to create style element");
            html_style
                .set_attribute("id", name)
                .expect("must set attribute");
            let html_style: web_sys::Node = html_style.unchecked_into();
            html_style.set_text_content(Some(style));
            let head = self.document.head().expect("must have a head");
            head.append_child(&html_style).expect("must append style");
        }  
    }
    
    pub(crate) fn mount_styles(&self, styles_map: HashMap<String, String>, is_first_mount: bool) -> Vec<(String, String)> {
        let mut styles_vec: Vec<(String, String)> = Vec::with_capacity(0);

        styles_map.iter().for_each(|(name, style)| {
            if self.styles.contains_key(name) {
                styles_vec.push((name.to_owned(), self.styles.get(name).unwrap().clone()));
            }
            else {
                styles_vec.push((name.to_owned(), style.to_owned()));
            }
        });
        if is_first_mount {
            self.base_styles.iter().for_each(|(name, style)| styles_vec.push((name.clone(), style.clone())));
        }

        styles_vec
    }

}