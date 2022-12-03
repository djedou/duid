use crate::core::{
    html::attributes::{Attribute}
};
use indextree::NodeId;

#[derive(Debug, Clone)]
pub(crate) enum Patch<MSG> {
    Replace(NodeId, NodeId),
    AddAttribute(NodeId, Attribute<MSG>),
    RemoveAttribute(NodeId, String),
    UpdateAttribute(NodeId, (String, Attribute<MSG>)),
    ChangeText(NodeId, Option<String>),
}