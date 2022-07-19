use indextree::{Arena, NodeId};
use crate::v_dom::html::attributes::Attribute;
use crate::v_dom::v_node::{NodeType, Node, Element, Leaf};
use crate::app::{Store};

pub type Vdom<T> = Arena<T>;



#[derive(Debug, Clone, PartialEq)]
pub struct Vnode
{
    pub tag: &'static str,
    pub namespace: Option<&'static str>,
    pub model: Option<Store>,
    pub node_type: NodeType,
    pub props: Vec<Attribute>,
    pub leaf_value: Option<&'static str>
}

impl Vnode {
    pub fn element(tag: &'static str, namespace: Option<&'static str>, model: Option<Store>, node_type: NodeType, props: Vec<Attribute>) -> Vnode {
        Vnode {
            tag,
            namespace,
            model,
            node_type,
            props,
            leaf_value: None
        }

    }

    pub fn fragment() -> Vnode {
        Vnode {
            tag: "fragment",
            namespace: None,
            model: None,
            node_type: NodeType::Fragment,
            props: vec![],
            leaf_value: None
        }
    }

    pub fn leaf(tag: &'static str, namespace: Option<&'static str>, model: Option<Store>, node_type: NodeType, props: Vec<Attribute>, value: &'static str) -> Vnode {
        Vnode {
            tag,
            namespace,
            model,
            node_type,
            props,
            leaf_value: Some(value)
        }
    }

    pub fn app() -> Vnode {
        Vnode {
            tag: "div",
            namespace: None,
            model: None,
            node_type: NodeType::Element,
            props: vec![],
            leaf_value: None
        }
    }
}



pub fn build_vdom(node: Node) -> (NodeId, Vdom<Vnode>) {
    let mut vdom: Vdom<Vnode> = Vdom::new();
    let app_node = Vnode::app();
    let app_node_id = vdom.new_node(app_node);
    
    match node {
        Node::Element(element) => {
            build_element(Some(app_node_id.clone()), &element, &mut vdom);
            (app_node_id, vdom)
        },
        Node::Fragment(fragment) => {
            build_fragment(Some(app_node_id.clone()), &fragment, &mut vdom);
            (app_node_id, vdom)
        },
        Node::Leaf(leaf) => {
            build_leaf(Some(app_node_id.clone()), &leaf, &mut vdom);
            (app_node_id, vdom)
        }
    }
}


fn build_element(parent_id: Option<NodeId>, element: &Element, vdom: &mut Vdom<Vnode>) -> NodeId {
    
    match parent_id {
        Some(parent) => {
            let vnode = Vnode::element(element.tag, element.namespace, element.model.clone(), element.node_type.clone(), element.props.clone());
            let vnode_id = vdom.new_node(vnode);
            parent.append(vnode_id, vdom);
            
            for child in &element.children {
                build_children(&vnode_id, child, vdom);
            }

            vnode_id
        },
        None => {
            let vnode = Vnode::element(element.tag, element.namespace, element.model.clone(), element.node_type.clone(), element.props.clone());
            let vnode_id = vdom.new_node(vnode);
        
            for child in &element.children {
                build_children(&vnode_id, child, vdom);
            }

            vnode_id
        }
    }

}

fn build_children(parent_id: &NodeId, node: &Node, vdom: &mut Vdom<Vnode>) {
    match node {
        Node::Element(element) => {
            build_element(Some(parent_id.clone()), element, vdom);
        },
        Node::Fragment(fragment) => {
            build_fragment(Some(parent_id.clone()), fragment, vdom);
        },
        Node::Leaf(leaf) => {
            build_leaf(Some(parent_id.clone()), &leaf, vdom);
        }
    }
}

fn build_fragment(parent_id: Option<NodeId>, nodes: &[Node], vdom: &mut Vdom<Vnode>) {
    
    
    match parent_id {
        Some(parent) => {
            let vnode = Vnode::fragment();
            let vnode_id = vdom.new_node(vnode);
            parent.append(vnode_id, vdom);
            
            for node in nodes {
                //build_element(Some(vnode_id), element, vdom);
                build_children(&vnode_id, &node, vdom);
            }
        },
        None => {
            let vnode = Vnode::fragment();
            let vnode_id = vdom.new_node(vnode);

            for node in nodes {
                //build_element(Some(vnode_id), element, vdom);
                build_children(&vnode_id, &node, vdom);
            }
        }
    }
}

fn build_leaf(parent_id: Option<NodeId>, leaf: &Leaf, vdom: &mut Vdom<Vnode>) {
    match parent_id {
        Some(parent) => {
            let vnode = Vnode::leaf(leaf.tag, leaf.namespace, leaf.model.clone(), leaf.node_type.clone(), leaf.props.clone(), leaf.value.clone());
            let vnode_id = vdom.new_node(vnode);
            parent.append(vnode_id, vdom);
        },
        None => {
            let vnode = Vnode::leaf(leaf.tag, leaf.namespace, leaf.model.clone(), leaf.node_type.clone(), leaf.props.clone(), leaf.value.clone());
            let _ = vdom.new_node(vnode);
        }
    }
}