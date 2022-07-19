use std::fmt::Debug;
use crate::v_dom::{
    html::attributes::AttributeValue
};
/// These are the plain attributes of an element
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub namespace: Option<&'static str>,
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub name: &'static str,
    /// the attribute value, which could be a simple value, and event or a function call
    pub value: Vec<AttributeValue>,
}

impl Attribute {
    /// create a plain attribute with namespace
    pub fn new(namespace: Option<&'static str>, name: &'static str, value: AttributeValue) -> Self {
        Attribute {
            namespace,
            name,
            value: vec![value]
        }
    }

    /// create from multiple values
    pub fn with_multiple_values(
        namespace: Option<&'static str>,
        name: &'static str,
        value: &[AttributeValue],
    ) -> Self {
        Attribute {
            name,
            value: value.to_vec(),
            namespace,
        }
    }

    /// return the name of this attribute
    pub fn name(&self) -> &'static str {
        &self.name
    }

    /// return the value of this attribute
    pub fn value(&self) -> &[AttributeValue] {
        &self.value
    }

    /// return the namespace of this attribute
    pub fn namespace(&self) -> Option<&'static str> {
        self.namespace
    }
}


#[inline]
pub fn attr(name: &'static str, value: AttributeValue) -> Attribute
{
    attr_ns(None, name, value)
}


#[inline]
pub fn attr_ns(
    namespace: Option<&'static str>,
    name: &'static str,
    value: AttributeValue,
) -> Attribute
{
    Attribute::new(namespace, name, value)
}

/// merge the values of attributes with the same name
#[doc(hidden)]
pub fn merge_attributes_of_same_name(
    attributes: &[&Attribute],
) -> Vec<Attribute>
{
    let mut merged: Vec<Attribute> = vec![];
    for att in attributes {
        if let Some(existing) =
            merged.iter_mut().find(|m_att| m_att.name == att.name)
        {
            existing.value.extend(att.value.clone());
        } else {
            merged.push(Attribute {
                namespace: None,
                name: att.name.clone(),
                value: att.value.clone(),
            });
        }
    }
    merged
}

/// group attributes of the same name
#[doc(hidden)]
pub fn group_attributes_per_name(
    attributes: &[Attribute],
) -> Vec<(&'static str, Vec<&Attribute>)>
{
    let mut grouped: Vec<(&'static str, Vec<&Attribute>)> = vec![];
    for attr in attributes {
        if let Some(existing) = grouped
            .iter_mut()
            .find(|(g_att, _)| *g_att == attr.name)
            .map(|(_, attr)| attr)
        {
            existing.push(attr);
        } else {
            grouped.push((&attr.name, vec![attr]))
        }
    }
    grouped
}