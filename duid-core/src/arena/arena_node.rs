use std::rc::Rc;
use std::cell::RefCell;
use super::NodeId;
use crate::core::{
    ActiveClosure,
    v_node::VirtualNodeType,
    html::attributes::Attribute
};


#[derive(Debug, Clone)]
pub(crate) struct ArenaNode<MSG> {
    pub(crate) id: NodeId,
    pub(crate) tag: &'static str,
    pub(crate) node_type: VirtualNodeType,
    pub(crate) namespace: Option<&'static str>,
    pub(crate) props: Vec<Attribute<MSG>>,
    pub(crate) value: Option<String>,
    pub(crate) active_closures: Rc<RefCell<ActiveClosure>>
}