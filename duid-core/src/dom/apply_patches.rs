use crate::arena::{Arena, ArenaNode, NodeId, ArenaNodeState};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use web_sys::{Document, Element, Node};
use crate::{
    core::{
        duid_events::Dispatch,
        v_node::VirtualNodeType,
    },
    dom::HtmlNodeBuilder
};
use wasm_bindgen::JsCast;



pub(crate) fn apply_patches<DSP, MSG>(
    old_arena: &mut Arena<ArenaNode<MSG>>, 
    new_arena: &mut Arena<ArenaNode<MSG>>,
    program: &DSP,
    doc: &Document, 
    styles_map: &mut HashMap<String, String>, 
    selectors_set: &mut HashMap<usize, HashSet<String>>
) 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
    DSP: Dispatch<MSG> + Clone + 'static
{
    let old_levels: Vec<(usize, Vec<NodeId>)> = old_arena.get_nodes_ids_by_levels_for_patching();
    crate::console::info!("levels after patches: {:#?}", old_levels);
    /*
    old_levels.iter().for_each(|(level, ids)| {
        ids.iter().for_each(|id| {
            match id.get_node_by_id_mut(old_arena) {
                Some(old_node) => {
                    let old_node_type = old_node.node_type.clone();
                    match &old_node.node_state.clone() {
                        ArenaNodeState::ReplaceOnlyData(new_node_id) => {
                            let new_html_node = new_arena.build_html_node(
                                new_node_id.clone(),
                                program,
                                &doc, 
                                styles_map, 
                                selectors_set
                            );

                            if let Some(new_node) = new_node_id.get_node_by_id(&new_arena) {
                                old_node.update_data_from(&new_node);
                            };
                            //mark_children_added_state::<MSG>(&[new_node_id.clone()], old_arena, new_arena);
                            replace_node(&old_node_type, &id, &old_arena.node_id_pairs, &doc, &new_html_node);
                        },
                        ArenaNodeState::ReplaceBy(node_id) => {
                            let new_html_node = new_arena.build_html_node(
                                node_id.clone(),
                                program,
                                &doc, 
                                styles_map, 
                                selectors_set
                            );

                            //mark_inserted_state(node_id.clone(), old_arena, &new_arena);
                            //mark_children_added_state::<MSG>(&[node_id.clone()], old_arena, new_arena);
                            replace_node(&old_node_type, &id, &old_arena.node_id_pairs, &doc, &new_html_node);
                        },
                        ArenaNodeState::IdChanged(node_id) => {
                            old_node.id = node_id.clone();
                            if let Some(pair) = id.clone().get_pair_mut(&mut old_arena.node_id_pairs) {
                                pair[1] = node_id.clone();
                            }
                            match old_node_type {
                                VirtualNodeType::Text |
                                VirtualNodeType::Comment |
                                VirtualNodeType::DocType |
                                VirtualNodeType::Fragment
                                => {},
                                VirtualNodeType::Element => {
                                    let element: Element = 
                                        doc.query_selector(&format!("[duid-id=\"{}\"]", id.value.clone()))
                                            .expect(&format!("Unable to get element with id {}", id.value.clone()))
                                            .expect(&format!("Unable to get element with id {}", id.value.clone()));
                                    
                                    let _ = element.remove_attribute("duid-id");
                                    let _ = element.set_attribute("duid-id", &format!("{}", node_id.value.clone()));
                                }
                            }

                        },
                        ArenaNodeState::Inserted(node_id) => {
                            let new_html_node = new_arena.build_html_node(
                                node_id.clone(),
                                program,
                                &doc, 
                                styles_map, 
                                selectors_set
                            );

                            //mark_children_added_state::<MSG>(&[node_id.clone()], old_arena, new_arena);
                            append_child_node(&id, &old_arena.node_id_pairs, &doc, &new_html_node);
                        },
                        _ => {}
                    }
                },
                None => {}
            }
        });
    });*/
}
/*
fn replace_node(
    node_type: &VirtualNodeType, 
    id: &NodeId, 
    ids: &[[NodeId; 2]], 
    doc: &Document,
    html_node: &Node) {
    match node_type {
        VirtualNodeType::Text |
        VirtualNodeType::Comment |
        VirtualNodeType::DocType
        => {
            if let Some((parent_id, index)) = id.get_index_in_parent_children(&ids) {
                let parent_element: Element = 
                    doc.query_selector(&format!("[duid-id=\"{}\"]", parent_id.value.clone()))
                    .expect("Unable to get element")
                    .expect("Unable to get element");
                
                if let Some(old_text_node) = parent_element.child_nodes().get(index as u32) {
                    let old_text_element: Element = old_text_node.unchecked_into();
                    let _ = old_text_element
                        .replace_with_with_node_1(&html_node)
                        .expect("Could not append child to mount");
                }
            }
        },
        VirtualNodeType::Fragment => {},
        VirtualNodeType::Element => {
            let old_element: Element = 
                    doc.query_selector(&format!("[duid-id=\"{}\"]", id.value.clone()))
                    .expect("Unable to get element")
                    .expect("Unable to get element");
                
            let _ = old_element
                .replace_with_with_node_1(&html_node)
                .expect("Could not append child to mount");
        }
    }
}

fn append_child_node(
    id: &NodeId, 
    ids: &[[NodeId; 2]], 
    doc: &Document,
    html_node: &Node) 
{
    if let Some(parent_id) = id.get_parent(&ids) {
        let parent_element: Element = 
            doc.query_selector(&format!("[duid-id=\"{}\"]", parent_id.value.clone()))
            .expect("Unable to get element")
            .expect("Unable to get element");
        
        let _ = parent_element.append_child(&html_node);
    }
}

*/