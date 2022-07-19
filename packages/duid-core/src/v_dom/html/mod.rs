//! Provides functions and macros to build html elements
use crate::v_dom::v_node::{LeafType, NodeType, create_fragment, create_leaf, Node};
use crate::app::Store;
use crate::v_dom::html::attributes::Attribute;
#[macro_use]
pub mod attributes;
pub mod tags;


pub use tags::{commons::*, self_closing::*};

/// Creates an html [text](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/text) element
pub fn text(model: Option<Store>, props: &[Attribute], value: &'static str) -> Node {
    create_leaf(
        None, 
        "text", 
        NodeType::Leaf(LeafType::Text), 
        model, 
        props, 
        value
    )
}

/// Creates an html [comment](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/comment) element
pub fn comment(value: &'static str) -> Node {
    create_leaf(
        None, 
        "comment", 
        NodeType::Leaf(LeafType::Comment), 
        None, 
        &[], 
        value
    )
}

/// Creates an html [text](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fragment) element
pub fn fragment<MSG>(nodes: &[Node]) -> Node {
    create_fragment(nodes)
}

/// create a doctype
pub fn doctype(s: impl ToString) -> Node {
    create_leaf(
        None, 
        "document-type", 
        NodeType::Leaf(LeafType::DocType), 
        None, 
        &[], 
        ""
    )
}
