use crate::arena::{Arena, ArenaNode};
use super::Patch;
use std::collections::{HashMap, HashSet};
use web_sys::{Document, Element};
use crate::{
    core::{
        duid_events::Dispatch,
    },
};
use wasm_bindgen::JsCast;



pub(crate) fn apply_patches<DSP, MSG>(
    old_arena: &mut Arena<ArenaNode<MSG>>,
    patches: &[Patch<MSG>],
    program: &DSP,
    doc: &Document, 
    styles_map: &mut HashMap<String, String>, 
    selectors_set: &mut HashMap<usize, HashSet<String>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
    DSP: Dispatch<MSG> + Clone + 'static
{
    // Step 1: delete all removed
    patches.iter().for_each(|patch| {
        match patch {
            Patch::Removed(node_id) => {
                old_arena.remove(node_id);
            },
            _ => {}
        }
    });

    // 
    patches.iter().for_each(|patch| {
        match patch {
            Patch::ValueChanged(id, value) => {
                if let Some((parent_id, index)) = id.get_index_in_parent_children(&old_arena.nodes_ids) {
                    let parent_element: Element = 
                        doc.query_selector(&format!("[duid-id=\"{}\"]", parent_id.get_duid_id()))
                        .expect("Unable to get element")
                        .expect("Unable to get element");
                    
                    let Some(ref new_value) = value else {
                        panic!("we did not get id {} node's value", id.get_duid_id());
                    };

                    let text_node: Element = doc.create_text_node(&new_value).unchecked_into();
                    
                    if let Some(old_text_node) = parent_element.child_nodes().get(index as u32) {
                        let old_text_element: Element = old_text_node.unchecked_into();
                        let _ = old_text_element
                            .replace_with_with_node_1(&text_node)
                            .expect("Could not append child to mount");
                    }
                }
            },
            Patch::Replacing(old_id, new_id, pairs, nodes) => {
                old_arena.add(pairs.to_owned(), &nodes);

                let new_html_node = old_arena.build_html_node(
                    new_id.clone(),
                    program,
                    &doc, 
                    styles_map,
                    selectors_set
                );

                let old_element: Element = 
                    doc.query_selector(&format!("[duid-id=\"{}\"]", old_id.get_duid_id()))
                    .expect(&format!("Unable to get element with id: {} to be replace with: {}", old_id.get_duid_id(), new_id.get_duid_id()))
                    .expect(&format!("Unable to get element with id: {} to be replace with: {}", old_id.get_duid_id(), new_id.get_duid_id()));
                
            let _ = old_element
                .replace_with_with_node_1(&new_html_node)
                .expect("Could not append child to mount");
            },
            _ => {}
        }
    });
}
