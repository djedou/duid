use std::fmt::Debug;
use crate::core::html::attributes::AttributeValue;

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute<MSG> {
    pub namespace: Option<&'static str>,
    pub name: &'static str,
    pub value: Vec<AttributeValue<MSG>>,
}

impl<MSG> Attribute<MSG> {
    pub fn new(namespace: Option<&'static str>, name: &'static str, value: AttributeValue<MSG>) -> Self {
        Attribute {
            namespace,
            name,
            value: vec![value]
        }
    }

    pub fn with_multiple_values(
        namespace: Option<&'static str>,
        name: &'static str,
        value: &[AttributeValue<MSG>],
    ) -> Self {
        Attribute {
            name,
            value: value.to_vec(),
            namespace,
        }
    }

    pub fn name(&self) -> &'static str {
        &self.name
    }

    pub fn value(&self) -> &[AttributeValue<MSG>] {
        &self.value
    }

    pub fn namespace(&self) -> Option<&'static str> {
        self.namespace
    }
}


#[inline]
pub fn attr<MSG>(name: &'static str, value: AttributeValue<MSG>) -> Attribute<MSG>
{
    attr_ns(None, name, value)
}


#[inline]
pub fn attr_ns<MSG>(
    namespace: Option<&'static str>,
    name: &'static str,
    value: AttributeValue<MSG>,
) -> Attribute<MSG>
{
    Attribute::new(namespace, name, value)
}

#[doc(hidden)]
pub fn merge_attributes_of_same_name<MSG>(
    attributes: &[&Attribute<MSG>],
) -> Vec<Attribute<MSG>>
{
    let mut merged: Vec<Attribute<MSG>> = vec![];
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

#[doc(hidden)]
pub fn group_attributes_per_name<MSG>(
    attributes: &[Attribute<MSG>],
) -> Vec<(&'static str, Vec<&Attribute<MSG>>)>
{
    let mut grouped: Vec<(&'static str, Vec<&Attribute<MSG>>)> = vec![];
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