use crate::core::html::{
    attributes::Attribute,
    nodes::{Node, Element, Leaf}
};


pub fn create_element<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, props: &[Attribute<MSG>], children: &[Node<MSG>]) -> Node<MSG> {
    
    Node::Element(Element {
        tag,
        namespace,
        props: props.to_vec(),
        children: children.to_vec()
    })
}


pub fn create_fragment<MSG: Clone>(nodes: &[Node<MSG>]) -> Node<MSG> {
    Node::Fragment(nodes.to_vec())
}

pub fn create_text_leaf<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, value: &str) -> Node<MSG> {
    
    Node::Text(Leaf {
        tag,
        namespace,
        props: Vec::<Attribute<MSG>>::with_capacity(0),
        value: value.to_owned()
    })
}

pub fn create_comment_leaf<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, value: &str) -> Node<MSG> {
    
    Node::Comment(Leaf {
        tag,
        namespace,
        props: Vec::<Attribute<MSG>>::with_capacity(0),
        value: value.to_owned()
    })
}

pub fn create_doctype_leaf<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, value: &str) -> Node<MSG> {
    
    Node::DocType(Leaf {
        tag,
        namespace,
        props: Vec::<Attribute<MSG>>::with_capacity(0),
        value: value.to_owned()
    })
}

