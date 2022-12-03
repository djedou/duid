use crate::core::{
    html::{
        attributes::{AttributeValue, Attribute},
        nodes::{Node, Element, Leaf}
    },
    events::listener::Listener
};


pub trait NodeMapMsg<MSG>
where
    MSG: 'static,
{
    fn map_msg<F, MSG2>(self, func: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static;

    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Node<MSG2>
    where
        MSG2: 'static;
}


pub trait ElementMapMsg<MSG>
where
    MSG: 'static,
{
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Element<MSG2>
    where
        MSG2: 'static;
}

pub trait LeafMapMsg<MSG>
where
    MSG: 'static,
{
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Leaf<MSG2>
    where
        MSG2: 'static;
}


pub trait AttributeMapMsg<MSG>
where
    MSG: 'static,
{
    fn map_msg<F, MSG2>(self, func: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static;

    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Attribute<MSG2>
    where
        MSG2: 'static;
}

pub trait AttributeValueMapMsg<MSG>
where
    MSG: 'static,
{
    fn map_callback<MSG2>(
        self,
        cb: Listener<MSG, MSG2>,
    ) -> AttributeValue<MSG2>
    where
        MSG2: 'static;
}


impl<MSG> NodeMapMsg<MSG> for Node<MSG>
where
    MSG: 'static,
{
    fn map_msg<F, MSG2>(self, func: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let cb = Listener::from(func);
        self.map_callback(cb)
    }

    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Node<MSG2>
    where
        MSG2: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map_callback(cb)),
            Node::Fragment(frg) => Node::Fragment(frg.into_iter().map(|child| child.map_callback(cb.clone())).collect()),
            Node::Text(text) => Node::Text(text.map_callback(cb)),
            Node::Comment(comment) => Node::Comment(comment.map_callback(cb)),
            Node::DocType(doctype) => Node::DocType(doctype.map_callback(cb)),
        }
    }
}

impl<MSG> ElementMapMsg<MSG> for Element<MSG>
where
    MSG: 'static,
{
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Element<MSG2>
    where
        MSG2: 'static,
    {
        Element {
            namespace: self.namespace,
            tag: self.tag,
            props: self
                .props
                .into_iter()
                .map(|attr| attr.map_callback(cb.clone()))
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|child| child.map_callback(cb.clone()))
                .collect()
        }
    }
}

impl<MSG> AttributeMapMsg<MSG> for Attribute<MSG>
where
    MSG: 'static,
{
    fn map_msg<F, MSG2>(self, func: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let cb = Listener::from(func);
        self.map_callback(cb)
    }
    
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Attribute<MSG2>
    where
        MSG2: 'static,
    {
        Attribute {
            name: self.name,
            value: self
                .value
                .into_iter()
                .map(|v| v.map_callback(cb.clone()))
                .collect(),
            namespace: self.namespace,
        }
    }
}

impl<MSG> AttributeValueMapMsg<MSG> for AttributeValue<MSG>
where
    MSG: 'static,
{
    fn map_callback<MSG2>(
        self,
        cb: Listener<MSG, MSG2>,
    ) -> AttributeValue<MSG2>
    where
        MSG2: 'static,
    {
        match self {
            AttributeValue::FunctionCall(this) => {
                AttributeValue::FunctionCall(this)
            }
            AttributeValue::Simple(this) => AttributeValue::Simple(this),
            AttributeValue::Style(this) => AttributeValue::Style(this),
            AttributeValue::EventListener(this) => {
                AttributeValue::EventListener(this.map_callback(cb))
            }
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }
}

impl<MSG> LeafMapMsg<MSG> for Leaf<MSG>
where
    MSG: 'static,
{
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Leaf<MSG2>
    where
        MSG2: 'static,
    {
        Leaf {
            namespace: self.namespace,
            tag: self.tag,
            props: self
                .props
                .into_iter()
                .map(|attr| attr.map_callback(cb.clone()))
                .collect(),
            value: self.value
        }
    }
}
