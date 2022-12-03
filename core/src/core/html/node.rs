use super::nodes::{Element, Leaf};
use crate::core::html::attributes::Attribute;

#[derive(Debug, Clone, PartialEq)]
pub enum Node<MSG> {
    Element(Element<MSG>),
    Fragment(Vec<Node<MSG>>),
    Text(Leaf<MSG>),
    Comment(Leaf<MSG>),
    DocType(Leaf<MSG>),
}

impl<MSG> Node<MSG> {

    pub fn extend_attrs(&mut self, attrs: impl IntoIterator<Item = Attribute<MSG>>) {
        match self {
            Node::Element(e) => {e.props.extend(attrs);},
            Node::Fragment(_) => {},
            Node::Text(t) => {t.props.extend(attrs);},
            Node::Comment(c) => {c.props.extend(attrs);},
            Node::DocType(d) => {d.props.extend(attrs);},
        }
    }

    pub fn extend_children(&mut self, children: impl IntoIterator<Item = Node<MSG>>) {
        match self {
            Node::Element(e) => {e.children.extend(children);},
            _ => {},
        }
    }
}