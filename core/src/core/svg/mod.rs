pub mod attributes;
pub mod tags;
use crate::core::html::{
    attributes::Attribute,
    nodes::{create_element, Node}
};
pub use tags::commons::*;

pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";


pub fn svg_element<MSG: Clone>(
    tag: &'static str,
    attrs: &[Attribute<MSG>],
    children: &[Node<MSG>]
) -> Node<MSG>
{
    create_element(
        Some(SVG_NAMESPACE), 
        tag, 
        attrs,
        children
    )
}
