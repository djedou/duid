use crate::core::html::attributes::Value;
use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub name: String,
    pub value: Value,
}

impl Style {
    pub fn new(name: impl ToString, value: impl Into<Value>) -> Self {
        Style {
            name: name.to_string(),
            value: value.into(),
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name.len() == 0 {
            write!(f, "{}", self.value)
        }
        else {
            write!(f, "{}:{}", self.name, self.value)
        }
    }
}