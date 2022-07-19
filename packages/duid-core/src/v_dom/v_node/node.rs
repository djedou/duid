use super::{Element, Leaf};



#[derive(Debug, Clone, PartialEq)]
pub enum Node
{
    Element(Element),
    Fragment(Vec<Node>),
    Leaf(Leaf)
}

