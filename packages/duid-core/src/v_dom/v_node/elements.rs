use crate::v_dom::html::attributes::Attribute;
use super::{Node, NodeType};
use crate::app::Store;



#[derive(Debug, Clone, PartialEq)]
pub struct Element
{
    pub tag: &'static str,
    pub namespace: Option<&'static str>,
    pub model: Option<Store>,
    pub node_type: NodeType,
    pub props: Vec<Attribute>,
    pub children: Vec<Node>
}


#[derive(Debug, Clone, PartialEq)]
pub struct Leaf {
    pub tag: &'static str,
    pub namespace: Option<&'static str>,
    pub model: Option<Store>,
    pub node_type: NodeType,
    pub props: Vec<Attribute>,
    pub value: &'static str
}


pub fn create_element(namespace: Option<&'static str>, tag: &'static str, node_type: NodeType, model: Option<Store>, props: &[Attribute], children: &[Node]) -> Node {
    Node::Element(Element {
        tag,
        namespace,
        model,
        node_type,
        props: props.to_vec(),
        children: children.to_vec()
    })
}


pub fn create_fragment(fragment: &[Node]) -> Node {
    Node::Fragment(fragment.to_vec())
}


pub fn create_leaf(namespace: Option<&'static str>, tag: &'static str, node_type: NodeType, model: Option<Store>, props: &[Attribute], value: &'static str) -> Node {
    Node::Leaf(Leaf {
        tag,
        namespace,
        model,
        node_type,
        props: props.to_vec(),
        value
    })
}

