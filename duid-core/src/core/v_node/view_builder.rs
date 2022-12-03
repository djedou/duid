use indextree::{Arena, NodeId};
use crate::core::{
    html::nodes::Node,
    v_node::{VirtualNode, VirtualNodeType},
    ActiveClosure
};
use std::rc::Rc;
use std::cell::RefCell;


pub(crate) struct ViewBuilder;



impl ViewBuilder {

    pub(crate) fn build<MSG>(node: Node<MSG>, arena: &mut Arena::<VirtualNode<MSG>>) -> NodeId
    where 
        MSG: std::fmt::Debug + Clone + 'static
    {
        match node {
            Node::Element(el) => {
                let el_node = arena.new_node(VirtualNode {
                    tag: el.tag,
                    node_type: VirtualNodeType::Element,
                    namespace: el.namespace,
                    props: el.props,
                    value: None,
                    real_node: Rc::new(RefCell::new(None)),
                    active_closures: Rc::new(RefCell::new(ActiveClosure::with_capacity(0)))
                });

                el.children.iter().for_each(|child| Self::create_child(&el_node, child.to_owned(), arena));
                el_node
            },
            Node::Fragment(elements) => {
                let frag_node = arena.new_node(VirtualNode {
                    tag: "fragment",
                    node_type: VirtualNodeType::Fragment,
                    namespace: None,
                    props: Vec::with_capacity(0),
                    value: None,
                    real_node: Rc::new(RefCell::new(None)),
                    active_closures: Rc::new(RefCell::new(ActiveClosure::with_capacity(0)))
                });

                elements.iter().for_each(|child| Self::create_child(&frag_node, child.to_owned(), arena));
                frag_node
            },
            Node::Text(leaf) => {
                let text_node = arena.new_node(VirtualNode {
                    tag: leaf.tag,
                    node_type: VirtualNodeType::Text,
                    namespace: leaf.namespace,
                    props: leaf.props,
                    value: Some(leaf.value),
                    real_node: Rc::new(RefCell::new(None)),
                    active_closures: Rc::new(RefCell::new(ActiveClosure::with_capacity(0)))
                });
                text_node
            },
            Node::Comment(leaf) => {
                let com_node = arena.new_node(VirtualNode {
                    tag: leaf.tag,
                    node_type: VirtualNodeType::Comment,
                    namespace: leaf.namespace,
                    props: leaf.props,
                    value: Some(leaf.value),
                    real_node: Rc::new(RefCell::new(None)),
                    active_closures: Rc::new(RefCell::new(ActiveClosure::with_capacity(0)))
                });
                com_node
            },
            Node::DocType(leaf) => {
                let doc_node = arena.new_node(VirtualNode {
                    tag: leaf.tag,
                    node_type: VirtualNodeType::DocType,
                    namespace: leaf.namespace,
                    props: leaf.props,
                    value: Some(leaf.value),
                    real_node: Rc::new(RefCell::new(None)),
                    active_closures: Rc::new(RefCell::new(ActiveClosure::with_capacity(0)))
                });
                doc_node
            }
        }
    }
    
    fn create_child<MSG>(
        parent_node: &NodeId, 
        child: Node<MSG>, 
        arena: &mut Arena::<VirtualNode<MSG>>
    ) 
    where
        MSG: std::fmt::Debug + Clone + 'static,
    {
        let child_node = Self::build(child, arena);
        parent_node.append(child_node, arena);
    }
}