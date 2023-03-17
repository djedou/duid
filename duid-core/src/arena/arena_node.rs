use std::rc::Rc;
use std::cell::RefCell;
use super::NodeId;
use crate::core::{
    ActiveClosure,
    v_node::VirtualNodeType,
    html::attributes::Attribute
};

#[derive(Debug, Clone)]
pub(crate) enum ArenaNodeState {
    None,
    ReplaceOnlyData(NodeId),
    ReplaceBy(NodeId),
    Removed,
    Inserted,
    IdChanged(NodeId),
    UnChanged
}


#[derive(Debug, Clone)]
pub(crate) struct ArenaNode<MSG> 
where 
    MSG: Clone
{
    pub(crate) id: NodeId,
    pub(crate) tag: &'static str,
    pub(crate) node_type: VirtualNodeType,
    pub(crate) namespace: Option<&'static str>,
    pub(crate) props: Vec<Attribute<MSG>>,
    pub(crate) value: Option<String>,
    pub(crate) active_closures: Rc<RefCell<ActiveClosure>>,
    pub(crate) node_state: ArenaNodeState
}

impl<MSG> PartialEq<ArenaNode<MSG>> for ArenaNode<MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static 
{
    fn eq(&self, other: &ArenaNode<MSG>) -> bool {
        self.tag == other.tag &&
        self.node_type == other.node_type &&
        self.namespace == other.namespace &&
        self.props == other.props &&
        self.value == other.value
    }
}

impl<MSG> ArenaNode<MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static 
{
    pub(crate) fn make_copy(&self) -> Self {
        ArenaNode {
            id: self.id.clone(),
            tag: self.tag.clone(),
            node_type: self.node_type.clone(),
            namespace: self.namespace.clone(),
            props: self.props.clone(),
            value: self.value.clone(),
            active_closures: Rc::clone(&self.active_closures),
            node_state: self.node_state.clone()
        } 
    }
}