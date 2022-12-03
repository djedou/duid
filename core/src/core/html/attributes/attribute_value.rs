use crate::core::{
    html::attributes::{Style},
    events::Listener
};
use jss::Value;
use std::fmt::{self, Debug};


pub enum AttributeValue<MSG> {
    FunctionCall(Value),
    Simple(Value),
    Style(Vec<Style>),
    EventListener(Listener<MSG>),
    Empty,
}

impl<MSG> Clone for AttributeValue<MSG> {
    fn clone(&self) -> Self {
        match self {
            AttributeValue::FunctionCall(this) => {
                AttributeValue::FunctionCall(this.clone())
            }
            AttributeValue::Simple(this) => {
                AttributeValue::Simple(this.clone())
            }
            AttributeValue::Style(this) => AttributeValue::Style(this.clone()),
            AttributeValue::EventListener(this) => {
                AttributeValue::EventListener(this.clone())
            }
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }
}

impl<MSG> Debug for AttributeValue<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttributeValue::FunctionCall(this) => this.fmt(f),
            AttributeValue::Simple(this) => this.fmt(f),
            AttributeValue::Style(this) => this.fmt(f),
            AttributeValue::EventListener(this) => this.fmt(f),
            AttributeValue::Empty => write!(f, "Empty"),
        }
    }
}

impl<MSG> PartialEq for AttributeValue<MSG> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                AttributeValue::FunctionCall(this),
                AttributeValue::FunctionCall(other),
            ) => this == other,
            (AttributeValue::Simple(this), AttributeValue::Simple(other)) => {
                this == other
            }
            (AttributeValue::Style(this), AttributeValue::Style(other)) => {
                this == other
            }
            (
                AttributeValue::EventListener(this),
                AttributeValue::EventListener(other),
            ) => this == other,
            (AttributeValue::Empty, AttributeValue::Empty) => true,
            (_, _) => false,
        }
    }
}

impl<MSG> AttributeValue<MSG> {
    pub fn from_styles(styles: impl IntoIterator<Item = Style>) -> Self {
        AttributeValue::Style(styles.into_iter().collect())
    }

    pub fn from_value(value: Value) -> Self {
        AttributeValue::Simple(value)
    }

    pub fn function_call(value: Value) -> Self {
        AttributeValue::FunctionCall(value)
    }

    pub fn get_simple(&self) -> Option<&Value> {
        match self {
            AttributeValue::Simple(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_function_call_value(&self) -> Option<&Value> {
        match self {
            AttributeValue::FunctionCall(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_style(&self) -> bool {
        matches!(self, AttributeValue::Style(_))
    }

    pub fn as_event_listener(&self) -> Option<&Listener<MSG>> {
        match self {
            AttributeValue::EventListener(cb) => Some(cb),
            _ => None,
        }
    }

    pub fn as_style(&self) -> Option<&Vec<Style>> {
        match self {
            AttributeValue::Style(styles) => Some(styles),
            _ => None,
        }
    }

    pub fn is_function_call(&self) -> bool {
        matches!(self, AttributeValue::FunctionCall(_))
    }
    
    pub fn is_empty(&self) -> bool {
        matches!(self, AttributeValue::Empty)
    }
}