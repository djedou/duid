use crate::core::{
    html::nodes::Node,
    v_node::{VirtualNode, VirtualNodeType}
};


pub(crate) struct ViewBuilder;



impl ViewBuilder {

    pub(crate) fn build<MSG>(node: Node<MSG>) -> VirtualNode<MSG>
    where 
        MSG: std::fmt::Debug + Clone + 'static
    {
        match node {
            Node::Element(el) => {
                VirtualNode {
                    key: 0,
                    tag: el.tag,
                    node_type: VirtualNodeType::Element,
                    namespace: el.namespace,
                    props: el.props,
                    value: None,
                    children: el.children.iter().map(|child| ViewBuilder::build(child.to_owned())).collect::<Vec<_>>()
                }
            },
            Node::Fragment(elements) => {
                VirtualNode {
                    key: 0,
                    tag: "fragment",
                    node_type: VirtualNodeType::Fragment,
                    namespace: None,
                    props: Vec::with_capacity(0),
                    value: None,
                    children: elements.iter().map(|child| ViewBuilder::build(child.to_owned())).collect::<Vec<_>>()
                }
            },
            Node::Text(leaf) => {
                VirtualNode {
                    key: 0,
                    tag: leaf.tag,
                    node_type: VirtualNodeType::Text,
                    namespace: leaf.namespace,
                    props: leaf.props,
                    value: Some(leaf.value),
                    children: Vec::with_capacity(0)
                }
            },
            Node::Comment(leaf) => {
                VirtualNode {
                    key: 0,
                    tag: leaf.tag,
                    node_type: VirtualNodeType::Comment,
                    namespace: leaf.namespace,
                    props: leaf.props,
                    value: Some(leaf.value),
                    children: Vec::with_capacity(0)
                }
            },
            Node::DocType(leaf) => {
                VirtualNode {
                    key: 0,
                    tag: leaf.tag,
                    node_type: VirtualNodeType::DocType,
                    namespace: leaf.namespace,
                    props: leaf.props,
                    value: Some(leaf.value),
                    children: Vec::with_capacity(0)
                }
            }
        }
    }
}