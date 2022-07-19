//! Create html [attributes][0]
//!
//! [0]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
#[macro_use]
mod attribute_macros;
mod attribute_value;
mod style;
mod attribute;

//use crate::vdom;
use crate::v_dom::{
    events::Event,
};
pub use attribute_macros::*;
pub use attribute_value::AttributeValue;
pub use jss::Value;
//pub use special::{key, replace, skip, Special};
pub use style::Style;
pub use attribute::*;
use crate::v_dom::events::listener::Listener;


pub fn style(
    style_name: impl ToString,
    value: impl Into<Value>,
) -> Attribute {
    attr(
        "style",
        AttributeValue::from_styles([Style::new(style_name, value.into())]),
    )
}


pub fn styles(
    pairs: impl IntoIterator<Item = (impl ToString, impl Into<Value>)>,
) -> Attribute {
    let styles = pairs.into_iter().map(|(key, value)| {
        Style::new(key.to_string(), Into::<Value>::into(value))
    });
    attr("style", AttributeValue::from_styles(styles))
}

/// A helper function to build styles by accepting pairs
pub fn styles_values(
    pairs: impl IntoIterator<Item = (impl ToString, impl Into<Value>)>,
) -> Attribute {
    let styles = pairs
        .into_iter()
        .map(|(key, value)| Style::new(key.to_string(), value));
    attr("style", AttributeValue::from_styles(styles))
}


pub fn styles_flag(
    trio: impl IntoIterator<Item = (impl ToString, impl Into<Value>, bool)>,
) -> Attribute {
    let styles = trio.into_iter().filter_map(|(key, value, flag)| {
        if flag {
            Some(Style::new(key, value))
        } else {
            None
        }
    });
    attr("style", AttributeValue::from_styles(styles))
}


pub fn classes_flag(
    pair: impl IntoIterator<Item = (impl ToString, bool)>,
) -> Attribute {
    let class_list = pair.into_iter().filter_map(|(class, flag)| {
        if flag {
            Some(class.to_string())
        } else {
            None
        }
    });

    classes(class_list)
}


pub fn classes(
    class_list: impl IntoIterator<Item = impl ToString>,
) -> Attribute {
    let class_values: Vec<_> = class_list
        .into_iter()
        .map(|v| AttributeValue::from_value(Value::from(v.to_string()))).collect();

    Attribute::with_multiple_values(None, "class", &class_values)
}


pub fn class_namespaced(
    namespace: impl ToString,
    class_names: impl ToString,
) -> Attribute {
    class(jss::class_namespaced(namespace, class_names))
}


pub fn classes_flag_namespaced(
    namespace: impl ToString,
    pair: impl IntoIterator<Item = (impl ToString, bool)>,
) -> Attribute {
    let class_list = pair.into_iter().filter_map(|(class_name, flag)| {
        if flag {
            Some(jss::class_namespaced(namespace.to_string(), class_name))
        } else {
            None
        }
    });
    classes(class_list)
}


pub fn attrs_flag(
    trio: impl IntoIterator<Item = (&'static str, impl Into<Value>, bool)>,
) -> impl IntoIterator<Item = Attribute> {
    trio.into_iter().filter_map(|(key, value, flag)| {
        if flag {
            Some(into_attr(key, value.into()))
        } else {
            None
        }
    })
}


pub fn maybe_attr(
    name: &'static str,
    value: Option<impl Into<Value>>,
) -> Attribute {
    if let Some(value) = value {
        into_attr(name, value)
    } else {
        empty_attr()
    }
}


pub fn checked(is_checked: bool) -> Attribute {
    if is_checked {
        #[cfg(not(feature = "with-dom"))]
        {
            into_attr("checked", "checked")
        }
        #[cfg(feature = "with-dom")]
        {
            into_attr("checked", true)
        }
    } else {
        empty_attr()
    }
}


pub fn disabled(is_disabled: bool) -> Attribute {
    if is_disabled {
        into_attr("disabled", true)
    } else {
        empty_attr()
    }
}


pub fn inner_html<V>(inner_html: V) -> Attribute
where
    V: Into<Value> + Clone,
{
    attr(
        "inner_html",
        AttributeValue::function_call(inner_html.into()),
    )
}


pub fn focus(is_focus: bool) -> Attribute {
    into_attr("focus", is_focus)
}


pub fn into_attr<V: Into<Value>>(att: &'static str, v: V) -> Attribute {
    attr(att, AttributeValue::from_value(v.into()))
}


pub fn empty_attr() -> Attribute {
    attr("", AttributeValue::Empty)
}

/// merge the plain values
#[doc(hidden)]
pub(crate) fn merge_plain_attributes_values(
    attr_values: &[&AttributeValue],
) -> Option<String> {
    let plain_values: Vec<String> = attr_values
        .iter()
        .flat_map(|att_value| match att_value {
            AttributeValue::Simple(simple) => Some(simple.to_string()),
            AttributeValue::FunctionCall(fvalue) => Some(fvalue.to_string()),
            _ => None,
        })
        .collect();
    if !plain_values.is_empty() {
        Some(plain_values.join(" "))
    } else {
        None
    }
}

/// merge the styles
#[doc(hidden)]
pub(crate) fn merge_styles_attributes_values(
    attr_values: &[&AttributeValue],
) -> Option<String> {
    use std::fmt::Write;

    let styles_values: Vec<String> = attr_values
        .iter()
        .flat_map(|att_value| match att_value {
            AttributeValue::Style(styles) => {
                let mut style_str = String::new();
                styles.iter().for_each(|s| {
                    write!(style_str, "{};", s).expect("must write")
                });
                Some(style_str)
            }
            _ => None,
        })
        .collect();

    if !styles_values.is_empty() {
        Some(styles_values.join(" "))
    } else {
        None
    }
}

/// The Attributes partition into 4 different types
pub struct SegregatedAttributes<'a> {
    /// the listeners of the event listeners
    pub listeners: Vec<&'a Listener<Event>>,
    /// plain attribute values
    pub plain_values: Vec<&'a AttributeValue>,
    /// style attribute values
    pub styles: Vec<&'a AttributeValue>,
    /// function calls
    pub function_calls: Vec<&'a AttributeValue>,
}

/// returns (listeners, plain_attribtues, function_calls)
#[doc(hidden)]
pub(crate) fn partition_callbacks_from_plain_styles_and_func_calls(
    attr: &Attribute,
) -> SegregatedAttributes {
    let mut listeners = vec![];
    let mut plain_values = vec![];
    let mut styles = vec![];
    let mut function_calls = vec![];
    for av in attr.value() {
        match av {
            AttributeValue::Simple(_plain) => {
                plain_values.push(av);
            }
            AttributeValue::FunctionCall(_call) => {
                function_calls.push(av);
            }
            AttributeValue::Style(_) => {
                styles.push(av);
            }
            AttributeValue::EventListener(cb) => {
                listeners.push(cb);
            }
            _ => (),
        }
    }
    SegregatedAttributes {
        listeners,
        plain_values,
        styles,
        function_calls,
    }
}