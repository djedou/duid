
#[macro_use]
mod attribute_macros;
mod attribute_value;
mod style;
mod attribute;

use crate::core::{
    events::Event,
    events::listener::Listener
};
pub use attribute_macros::*;
pub use attribute_value::AttributeValue;
pub use jss::Value;
pub use style::Style;
pub use attribute::*;


pub fn style<MSG>(
    style_name: &'static str,
    value: impl Into<Value>,
) -> Attribute<MSG> {
    attr(
        style_name,
        AttributeValue::from_styles([Style::new("", value.into())]),
    )
}

pub fn styles<MSG>(
    pairs: impl IntoIterator<Item = (impl ToString, impl Into<Value>)>,
) -> Attribute<MSG> {
    let styles = pairs.into_iter().map(|(key, value)| {
        Style::new(key.to_string(), Into::<Value>::into(value))
    });
    attr("style", AttributeValue::from_styles(styles))
}

pub fn styles_values<MSG>(
    pairs: impl IntoIterator<Item = (impl ToString, impl Into<Value>)>,
) -> Attribute<MSG> {
    let styles = pairs
        .into_iter()
        .map(|(key, value)| Style::new(key.to_string(), value));
    attr("style", AttributeValue::from_styles(styles))
}


pub fn styles_flag<MSG>(
    trio: impl IntoIterator<Item = (impl ToString, impl Into<Value>, bool)>,
) -> Attribute<MSG> {
    let styles = trio.into_iter().filter_map(|(key, value, flag)| {
        if flag {
            Some(Style::new(key, value))
        } else {
            None
        }
    });
    attr("style", AttributeValue::from_styles(styles))
}


pub fn classes_flag<MSG>(
    pair: impl IntoIterator<Item = (impl ToString, bool)>,
) -> Attribute<MSG> {
    let class_list = pair.into_iter().filter_map(|(class, flag)| {
        if flag {
            Some(class.to_string())
        } else {
            None
        }
    });

    classes(class_list)
}


pub fn classes<MSG>(
    class_list: impl IntoIterator<Item = impl ToString>,
) -> Attribute<MSG> {
    let class_values: Vec<_> = class_list
        .into_iter()
        .map(|v| AttributeValue::from_value(Value::from(v.to_string()))).collect();

    Attribute::with_multiple_values(None, "class", &class_values)
}

pub fn selectors<MSG>(
    selector_list: impl IntoIterator<Item = impl ToString>,
) -> Attribute<MSG> {
    let selector_values: Vec<_> = selector_list
        .into_iter()
        .map(|v| v.to_string()).collect();
    
    let values: String = selector_values.join(";");
    Attribute::new(None, "selectors", AttributeValue::from_value(Value::from(values)))
}


pub fn class_namespaced<MSG>(
    namespace: impl ToString,
    class_names: impl ToString,
) -> Attribute<MSG> {
    class(jss::class_namespaced(namespace, class_names))
}


pub fn classes_flag_namespaced<MSG>(
    namespace: impl ToString,
    pair: impl IntoIterator<Item = (impl ToString, bool)>,
) -> Attribute<MSG> {
    let class_list = pair.into_iter().filter_map(|(class_name, flag)| {
        if flag {
            Some(jss::class_namespaced(namespace.to_string(), class_name))
        } else {
            None
        }
    });
    classes(class_list)
}


pub fn attrs_flag<MSG>(
    trio: impl IntoIterator<Item = (&'static str, impl Into<Value>, bool)>,
) -> impl IntoIterator<Item = Attribute<MSG>> {
    trio.into_iter().filter_map(|(key, value, flag)| {
        if flag {
            Some(into_attr(key, value.into()))
        } else {
            None
        }
    })
}


pub fn maybe_attr<MSG>(
    name: &'static str,
    value: Option<impl Into<Value>>,
) -> Attribute<MSG> {
    if let Some(value) = value {
        into_attr(name, value)
    } else {
        empty_attr()
    }
}


pub fn checked<MSG>(is_checked: bool) -> Attribute<MSG> {
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


pub fn disabled<MSG>(is_disabled: bool) -> Attribute<MSG> {
    if is_disabled {
        into_attr("disabled", true)
    } else {
        empty_attr()
    }
}


pub fn inner_html<V, MSG>(inner_html: V) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
{
    attr(
        "inner_html",
        AttributeValue::function_call(inner_html.into()),
    )
}


pub fn focus<MSG>(is_focus: bool) -> Attribute<MSG> {
    into_attr("focus", is_focus)
}


pub fn into_attr<V: Into<Value>, MSG>(att: &'static str, v: V) -> Attribute<MSG> {
    attr(att, AttributeValue::from_value(v.into()))
}


pub fn empty_attr<MSG>() -> Attribute<MSG> {
    attr("", AttributeValue::Empty)
}

#[doc(hidden)]
pub(crate) fn merge_plain_attributes_values<MSG>(
    attr_values: &[&AttributeValue<MSG>],
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

#[doc(hidden)]
pub(crate) fn merge_styles_attributes_values<MSG>(
    attr_values: &[&AttributeValue<MSG>],
) -> Option<String> {
    use std::fmt::Write;

    let styles_values: Vec<String> = attr_values
        .iter()
        .flat_map(|att_value| match att_value {
            AttributeValue::Style(styles) => {
                let mut style_str = String::new();
                styles.iter().for_each(|s| {
                    write!(style_str, "{}", s).expect("must write")
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

pub struct SegregatedAttributes<'a, MSG> {
    pub listeners: Vec<&'a Listener<Event, MSG>>,
    pub plain_values: Vec<&'a AttributeValue<MSG>>,
    pub styles: Vec<&'a AttributeValue<MSG>>,
    pub function_calls: Vec<&'a AttributeValue<MSG>>,
}

#[doc(hidden)]
pub(crate) fn partition_callbacks_from_plain_styles_and_func_calls<MSG>(
    attr: &Attribute<MSG>,
) -> SegregatedAttributes<MSG> {
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