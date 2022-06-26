//! Provides functions and macros to build html elements
use crate::v_dom::v_node::{
    leaf, NodeTrait, Attribute, Node
};
pub use mt_dom::{element, element_ns};

#[macro_use]
pub mod attributes;
pub mod tags;

/*
#[cfg(feature = "with-dom")]
pub use crate::events;
*/
pub use tags::{commons::*, self_closing::*};


pub fn view_if<MSG>(flag: bool, node: Node<MSG>) -> Node<MSG> {
    if flag {
        node
    } else {
        comment("hidden")
    }
}


#[inline]
pub fn html_element<MSG>(
    namespace: Option<&'static str>,
    tag: &'static str,
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
    self_closing: bool,
) -> Node<MSG> {
    // we do a correction to children where text node siblings are next to each other by inserting
    // a comment separator in between them, to prevent the browser from merging the 2 text node
    // together
    let mut corrected_children: Vec<Node<MSG>> = vec![];
    for child in children {
        if let Some(last) = corrected_children.last() {
            if last.is_text() {
                corrected_children.push(comment("separator"));
            }
        }
        corrected_children.push(child);
    }
    element_ns(namespace, tag, attrs, corrected_children, self_closing)
}


#[macro_export]
macro_rules! text {
    ( $($arg: tt)* ) => {
        $crate::html::text(format!($($arg)*))
    };
}


pub fn text<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::text(s))
}


pub fn safe_html<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::safe_html(s))
}


pub fn comment<S, MSG>(s: S) -> Node<MSG>
where
    S: ToString,
{
    Node::Leaf(leaf::comment(s))
}


pub fn fragment<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Node<MSG> {
    Node::Leaf(leaf::fragment(nodes))
}

/// create a doctype
pub fn doctype<MSG>(s: impl ToString) -> Node<MSG> {
    Node::Leaf(leaf::doctype(s))
}