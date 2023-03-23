mod dom_diff;
mod patches;
mod build_html_node;
mod apply_patches;
mod patches_data;

pub(crate) use dom_diff::*;
pub(crate) use patches::*;
pub(crate) use build_html_node::*;
pub(crate) use apply_patches::*;
pub(crate) use patches_data::*;

use crate::arena::ArenaNode;
use wasm_bindgen::JsCast;
use std::fmt::Debug;
use web_sys::{
    self, Element, Node, Document
};
use crate::core::{
    v_node::VirtualNode,
    util::document,
    duid_events::Dispatch,
};
use crate::arena::{Arena, NodeId};
use crate::tailwindcss_system::builder::StyleContainer;
use std::collections::{HashMap, HashSet};




#[derive(Debug, Clone)]
pub(crate) struct Dom<MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static
{
    pub(crate) mount_node: Node,
    pub(crate) replace: bool,
    pub(crate) use_shadow: bool,
    pub(crate) arena: Arena<ArenaNode<MSG>>,
    pub(crate) real_node: Option<Node>,
    pub(crate) document: Document,
    pub(crate) base_styles: HashMap<String, String>,
    pub(crate) styles: HashMap<String, String>,
    pub(crate) style_container: StyleContainer
}


impl<MSG> Dom<MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static
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
            arena: Arena::new(),
            replace,
            use_shadow, 
            real_node: None,
            document: doc,
            base_styles,
            styles,
            style_container: StyleContainer::new()
        }
    }
}

impl<MSG> Dom<MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static
{
    pub(crate) fn mount<DSP>(&mut self, program: &DSP, root_node: VirtualNode<MSG>) 
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let mut style_map: HashMap<String, String> = HashMap::with_capacity(0); 
        let mut selectors_set: HashMap<usize, HashSet<String>> = HashMap::with_capacity(0);
        // step 1: build Arena
        let mut arena = Arena::new_from_virtual_node(&root_node);
        self.real_node = Some(arena.build_html_node::<DSP>(
            arena.get_first_id(), 
            program, 
            &self.document, 
            &mut style_map, 
            &mut selectors_set
        ));
        self.arena = arena;
        let tailwind_styles = self.style_container.build(&selectors_set);
        self.first_mount();
        self.inject_styles(&self.mount_styles(style_map, true));
        self.inject_styles(&tailwind_styles);
    }

    pub(crate) fn render<DSP>(&mut self, program: &DSP, new_root_node: VirtualNode<MSG>) 
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let mut style_map: HashMap<String, String> = HashMap::with_capacity(0);
        let mut selectors_set: HashMap<usize, HashSet<String>> = HashMap::with_capacity(0);
        // step 1: build a new Arena
        let mut arena = Arena::new_from_virtual_node(&new_root_node);
        // step 2: patches
        //crate::console::info!("before patches: {:#?}", self.arena);
        let new_patches = patches(&mut self.arena, &mut arena);
        crate::console::info!("after patches: {:#?}", self.arena);   
         
        apply_patches(
            &mut self.arena,
            &new_patches,
            program, 
            &self.document, 
            &mut style_map, 
            &mut selectors_set);
        
        crate::console::info!("after apply patches: {:#?}", self.arena); 
        self.arena.clean_patches();
/*
        //crate::console::info!("clean_patches: {:#?}", self.arena); 
            
        crate::console::info!("after patches: {:#?}", self.arena);
        
        let _ = self.root_node.set_key(1);
        let patches = diff(&self.root_node, &new_root_node);
        crate::console::info!("patches: {:#?}", patches);


        new_root_node.build_node(program, &self.document, &mut style_map, &mut selectors_set);

        self.render_new_node(new_root_node);
        let tailwind_styles = self.style_container.build(&selectors_set);
        self.inject_styles(&self.mount_styles(style_map, false));
        self.inject_styles(&tailwind_styles);
        */
    }

    fn first_mount(&self) {
        
        if let Some(node) = &self.real_node {
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
        
    }

    fn render_new_node(&mut self, new_root_node: VirtualNode<MSG>) {
        /*
        let mut updated = false;
        match (&*self.root_node.real_node.borrow(), &*new_root_node.real_node.borrow()) {
            (Some(old_node), Some(new_node)) => {
                let mount_element: &Element = old_node.unchecked_ref();
                mount_element
                    .replace_with_with_node_1(new_node)
                    .expect("Could not append child to mount");

                    updated = true;
                },
                _ => {}
            };
            
        if updated {
            self.root_node = new_root_node;
        }
        */
    }

    fn inject_styles(&self, styles: &[(String, String)]) {
        for (name, style) in styles.iter() {
            let blank = style.trim().is_empty();
            if blank {
                continue;
            }
            else {
                self.inject_style(name, style);
            }
        }
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
        self.style_container.themes.variables.iter().for_each(|(name, style)| styles_vec.push((name.clone(), style.clone())));
        self.style_container.themes.mode_variables.iter().for_each(|(name, style)| styles_vec.push((name.clone(), style.clone())));

        styles_vec
    }

}