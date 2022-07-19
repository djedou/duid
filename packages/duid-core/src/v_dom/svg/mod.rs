//! Provides functions and macros to build svg elements
pub mod attributes;
pub mod tags;
use crate::v_dom::v_node;
use crate::v_dom::html::attributes::Attribute;

pub use tags::commons::*;

/// SVG namespace const, use this when creating an svg element dynamically in the DOM
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";


pub fn svg_element(
    tag: &'static str,
    model: Option<crate::app::Store>, 
    attrs: &[Attribute],
    children: &[v_node::Node],
) -> v_node::Node {
    crate::v_dom::v_node::create_element(Some(SVG_NAMESPACE), tag, crate::v_dom::v_node::NodeType::Element, model, attrs, children)
}