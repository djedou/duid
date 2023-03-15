/*
use crate::core::{
    html::{
        attributes::{Attribute}
    },
    v_node::VirtualNode
};


#[derive(Debug, Clone, PartialEq)]
pub enum PropsChangeType<MSG> {
    AddedProp(Attribute<MSG>),
    RemovedProp(String),
    UpdatedPropValues(String, Attribute<MSG>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType<MSG> {
    ChangedTag,
    ChangedNodeType,
    ChangedNamespace,
    ChangedProps(Vec<PropsChangeType<MSG>>),
    ChangedText(Option<String>),
    UnChanged
}



impl<MSG> VirtualNode<MSG> {
    pub(crate) fn diff(&self, other: &Self) -> ChangeType<MSG> {
        let mut change = ChangeType::UnChanged;
        if self.tag != other.tag {
            change = ChangeType::ChangedTag;
        }
        if self.node_type != other.node_type {
            change = ChangeType::ChangedNodeType;
        }
        if self.namespace != other.namespace {
            change = ChangeType::ChangedNamespace;
        }
        if self.value != other.value {
            change = ChangeType::ChangedText(other.value.clone());
        }

        let props_changes = self.diff_props(&other.props);
        if props_changes.len() > 0 {
            change = ChangeType::ChangedProps(props_changes);
        }

        change
    }

    fn diff_props(&self, new_attrs: &[Attribute<MSG>]) -> Vec<PropsChangeType<MSG>> {
        let mut changes = vec![];

        for old_attr in self.props.iter() {
            match new_attrs.iter().find(|n| n.name == old_attr.name) {
                Some(exist) => {
                    if old_attr.value.len() != exist.value.len() {
                        changes.push(PropsChangeType::UpdatedPropValues(old_attr.name.clone().to_owned(), Attribute::with_multiple_values(
                            exist.namespace.clone(),
                            exist.name,
                            &exist.value
                        )));
                    }

                    let mut there_is_changes = false;
                    for old_value in old_attr.value.iter() {
                        if !exist.value.contains(old_value) {
                            there_is_changes = true;
                            break;
                        }
                    }
                    if there_is_changes {
                        changes.push(PropsChangeType::UpdatedPropValues(old_attr.name.clone().to_owned(), Attribute::with_multiple_values(
                            exist.namespace.clone(),
                            exist.name,
                            &exist.value
                        )));
                    }
                },
                None => {
                    changes.push(PropsChangeType::RemovedProp(old_attr.name.clone().to_owned()));
                }
            }
        }

        for new_attr in new_attrs.iter() {
            match self.props.iter().find(|n| n.name == new_attr.name) {
                Some(_) => {},
                None => {
                    changes.push(PropsChangeType::AddedProp(Attribute::with_multiple_values(
                        new_attr.namespace.clone(),
                        new_attr.name,
                        &new_attr.value
                    )));
                }
            }
        }

        changes
    }
}
*/