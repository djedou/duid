use web_sys::{Element, Node, Document};
use std::rc::Rc;
use std::cell::RefCell;
use crate::{
    core::{
        v_node::VirtualNode,
        ActiveClosure,
        duid_events::Dispatch
    },
    dom::HtmlNodeBuilder
};
use std::collections::{HashMap, HashSet};

use std::cmp::Ordering;
use super::{NodeId, ArenaNode};

#[derive(Debug, Clone)]
pub struct Arena<T> {
    pub(crate) nodes: Vec<T>,
    pub(crate) first_node_id: NodeId,
    pub(crate) last_node_id: NodeId,
    pub(crate) node_id_pairs: Vec<[NodeId; 2]>
}

impl<MSG> Arena<ArenaNode<MSG>> 
where 
    MSG: std::fmt::Debug + Clone + PartialEq + 'static, 
{
    pub(crate) fn new() -> Arena<ArenaNode<MSG>> {
        Arena {
            nodes: Vec::with_capacity(0),
            first_node_id: NodeId::default(),
            last_node_id: NodeId::default(),
            node_id_pairs: Vec::with_capacity(0)
        }
    }

    pub(crate) fn get_first_node_id(&self) -> NodeId {
        self.first_node_id.clone()
    }

    pub(crate) fn new_from_virtual_node(virtual_node: &VirtualNode<MSG>) -> Arena<ArenaNode<MSG>> {
        let mut arena = Arena::new();
        arena.first_node_id = virtual_node.get_arena_node_id();
        arena.node_id_pairs = Arena::to_indexes_pair(&virtual_node);
        Arena::to_nodes(&virtual_node, &mut arena);
        arena
    }

    pub(crate) fn get_nodes_ids_by_levels(&self) -> Vec<(usize, Vec<NodeId>)> {
        let mut levels: Vec<(usize, Vec<NodeId>)> = vec![];
        self.first_node_id.get_levels(
            1, 
            &[self.first_node_id.clone()], 
            &mut levels,
            &self.node_id_pairs
        );

        levels
    }

    fn to_nodes(node: &VirtualNode<MSG>, arena: &mut Arena<ArenaNode<MSG>>) {
        
        arena.nodes.push(node.into_arena_node());

        let mut children = node.children.iter();
        
        while let Some(child) = children.next() {
            Arena::to_nodes(child, arena);
        }
    }
    
    fn to_indexes_pair(node: &VirtualNode<MSG>) -> Vec<[NodeId; 2]> {
        let result: Vec<_> = node.children.iter().map(|child| {
            let mut indexes: Vec<[NodeId; 2]> = vec![[node.get_arena_node_id(), child.get_arena_node_id()]];
            indexes.extend_from_slice(&Arena::to_indexes_pair(child));
            indexes
        })
        .collect();

        let mut res = result.into_iter()
            .fold(vec![], |mut old_vec: Vec<[NodeId; 2]>, new_vec: Vec<[NodeId; 2]>| {
                old_vec.extend_from_slice(&new_vec);
                old_vec
            });
            
        res.sort_by(|a, b| {
            match a[0].value.cmp(&b[0].value).is_lt() {
                true => Ordering::Less,
                false => a[1].value.cmp(&b[1].value)
            }
        });

        res
    }

    pub(crate) fn build_html_node<DSP>(
        &mut self, 
        node_id: NodeId,
        program: &DSP,
        doc: &Document, 
        styles_map: &mut HashMap<String, String>, 
        selectors_set: &mut HashMap<usize, HashSet<String>>
    ) -> Node
    where
        DSP: Dispatch<MSG> + Clone + 'static
    {
        // get ArenaNode by nodeId
        match node_id.get_node_by_id_mut(self) {
            Some(node) => {
                let html_node = HtmlNodeBuilder::build(
                    program, 
                    &doc, 
                    node,
                    styles_map,
                    selectors_set
                );
                // get children
                let children_ids = node_id.get_children(&self.node_id_pairs); 
                let children_nodes: Vec<_> = children_ids.iter().map(|child| self.build_html_node(
                    child.clone(),
                    program,
                    &doc, 
                    styles_map,
                    selectors_set
                ))
                .collect();

                children_nodes.iter().for_each(|child| {
                    let _ = html_node.append_child(&child);
                });
                html_node
            },
            None => todo!()
        }
    }
}