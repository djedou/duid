use crate::core::html::{
    attributes::Attribute,
    nodes::Node
};



#[derive(Debug, Clone, PartialEq)]
pub struct Element<MSG>
{
    pub tag: &'static str,
    pub namespace: Option<&'static str>,
    pub props: Vec<Attribute<MSG>>,
    pub children: Vec<Node<MSG>>
}


#[derive(Debug, Clone, PartialEq)]
pub struct Leaf<MSG> {
    pub tag: &'static str,
    pub namespace: Option<&'static str>,
    pub props: Vec<Attribute<MSG>>,
    pub value: String,
}