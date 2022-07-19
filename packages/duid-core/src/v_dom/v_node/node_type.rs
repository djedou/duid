

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element,
    Fragment,
    Leaf(LeafType)
}


#[derive(Debug, Clone, PartialEq)]
pub enum LeafType {
    Text,
    Comment,
    DocType
}
