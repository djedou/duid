mod elements;
mod node;
mod node_helpers;

#[macro_use]
pub mod attributes;
pub mod tags;

pub mod nodes {
    pub use super::elements::*;
    pub use super::node::*;
    pub use super::node_helpers::*;
}
pub use tags::{commons::*, self_closing::*};



pub fn text<MSG: Clone>(
    value: &str,
) -> nodes::Node<MSG>
{
    nodes::create_text_leaf(None, "text", value)
}


pub fn comment<MSG: Clone>(
    value: &'static str,
) -> nodes::Node<MSG>
{
    nodes::create_comment_leaf(None, "comment", value)
}

pub fn doctype<MSG: Clone>(
    value: &'static str,
) -> nodes::Node<MSG>
{
    nodes::create_doctype_leaf(None, "document-type", value)
}
