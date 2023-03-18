use std::rc::Rc;
use std::cell::RefCell;
use super::NodeId;
use crate::core::{
    ActiveClosure,
    v_node::VirtualNodeType,
    html::attributes::Attribute
};

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum ArenaNodeState {
    /*
    ReplaceOnlyData(NodeId),
    */
    Inserted,
    Added,
    Replacing(NodeId),
    ReplaceBy(NodeId),
    Removed,
    UnChanged,
    DataChanged(DataState),
    IdChanged(NodeId, NodeId), // (old, new)
    #[default]
    None,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum DataState {
    Tag,
    NodeType,
    Namespace,
    Props,
    Value,
    #[default]
    None,
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
    pub(crate) node_state: ArenaNodeState,
    pub(crate) update_props: Vec<Attribute<MSG>>,
    pub(crate) update_value: Option<String>,
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
    pub(crate) fn update_data_from(&mut self, other: &ArenaNode<MSG>) {
        self.tag = other.tag.clone();
        self.namespace = other.namespace.clone();
        self.props = other.props.clone();
        self.value = other.value.clone();
    }
}