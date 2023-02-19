use std::rc::Rc;
use std::cell::RefCell;
use crate::core::{
    ActiveClosure, DATA_VDOM_ID, create_unique_identifier,
    events::{Event, Listener},
    duid_events::Dispatch,
    html::attributes::{
        Attribute, merge_attributes_of_same_name, SegregatedAttributes, partition_callbacks_from_plain_styles_and_func_calls,
        merge_plain_attributes_values, merge_styles_attributes_values, AttributeValue
    }
};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    self, HtmlInputElement,EventTarget, Element, HtmlButtonElement, HtmlDataElement,
    HtmlDetailsElement, HtmlElement, HtmlFieldSetElement,
    HtmlLiElement, HtmlLinkElement, HtmlMenuItemElement, HtmlMeterElement,
    HtmlOptGroupElement, HtmlOptionElement, HtmlOutputElement,
    HtmlParamElement, HtmlProgressElement, HtmlSelectElement, HtmlStyleElement,
    HtmlTextAreaElement, Node, Document
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VirtualNodeType {
    Element,
    Fragment,
    Text,
    Comment,
    DocType,
}



#[derive(Debug, Clone)]
pub(crate) struct VirtualNode<MSG>
{
    pub(crate) tag: &'static str,
    pub(crate) node_type: VirtualNodeType,
    pub(crate) namespace: Option<&'static str>,
    pub(crate) props: Vec<Attribute<MSG>>,
    pub(crate) value: Option<String>,
    pub(crate) real_node: Rc<RefCell<Option<Node>>>,
    pub(crate) active_closures: Rc<RefCell<ActiveClosure>>,
    pub(crate) children: Vec<VirtualNode<MSG>>
}


impl<MSG> VirtualNode<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static 
{
    pub(crate) fn new() -> Self {
        VirtualNode {
            tag: "",
            node_type: VirtualNodeType::Element,
            namespace: None,
            props: Vec::with_capacity(0),
            value: None,
            real_node: Rc::new(RefCell::new(None)),
            active_closures: Rc::new(RefCell::new(ActiveClosure::with_capacity(0))),
            children: Vec::with_capacity(0)
        } 
    }

    pub(crate) fn replace_by(&mut self, new_node: &VirtualNode<MSG>) {

            if let (Some(old_node), Some(new_node)) = (&*self.real_node.borrow() , &*new_node.real_node.borrow()) {
                let old_element: &Element = old_node.unchecked_ref();
                old_element
                    .replace_with_with_node_1(new_node)
                    .expect("Could not append child to mount");
            }
            
            self.tag = new_node.tag;
            self.node_type = new_node.node_type.to_owned();
            self.namespace = new_node.namespace;
            self.props = new_node.props.to_owned();
            self.value = new_node.value.to_owned();
            self.real_node = new_node.real_node.to_owned();
            self.active_closures = new_node.active_closures.to_owned();
            self.children = new_node.children.to_owned();

    }

    pub(crate) fn build_node<DSP>(&self, program: &DSP, doc: &Document, styles_map: &mut HashMap<String, String>, selectors_set: &mut HashSet<String>) 
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        match &self.node_type {
            VirtualNodeType::Element => {
                let element = if let Some(namespace) = self.namespace.clone() {
                    doc
                        .create_element_ns(Some(&namespace), &self.tag)
                        .expect("Unable to create element")
                } else {
                    doc
                        .create_element(&self.tag)
                        .expect("Unable to create element")
                };
                let attrs = self.props.iter().map(|attr| attr).collect::<Vec<_>>();
                Self::set_element_attributes(program, &element, &attrs, &mut self.active_closures.borrow_mut(), styles_map, selectors_set);
                *self.real_node.borrow_mut() = Some(element.unchecked_into::<Node>());
                self.children.iter().for_each(|child| child.build_node(program, &doc, styles_map, selectors_set));
                
                if let Some(real_node_parent) = &*self.real_node.borrow() {
                    self.children.iter().for_each(|child| {
                        if let Some(real_node_child) = &*child.real_node.borrow() {
                            let _ = real_node_parent.append_child(&real_node_child);
                        }
                    });
                }
            },
            VirtualNodeType::Fragment => {
                let doc_fragment = doc.create_document_fragment();
                *self.real_node.borrow_mut() = Some(doc_fragment.unchecked_into::<Node>());
                self.children.iter().for_each(|child| child.build_node(program, &doc, styles_map, selectors_set));
                
                if let Some(real_node_parent) = &*self.real_node.borrow() {
                    self.children.iter().for_each(|child| {
                        if let Some(real_node_child) = &*child.real_node.borrow() {
                            let _ = real_node_parent.append_child(&real_node_child);
                        }
                    });
                }
            },
            VirtualNodeType::Text => {
                if let Some(value) = &self.value {
                    let attrs = self.props.iter().map(|attr| attr).collect::<Vec<_>>();
                    let text_node: Element = doc.create_text_node(value).unchecked_into();
                    Self::set_element_attributes(program, &text_node, &attrs, &mut self.active_closures.borrow_mut(), styles_map, selectors_set);
                    *self.real_node.borrow_mut() = Some(text_node.unchecked_into::<Node>());
                }
            },
            VirtualNodeType::Comment => {
                if let Some(value) = &self.value {
                    let comment_node: Element = doc.create_comment(value).unchecked_into();
                    *self.real_node.borrow_mut() = Some(comment_node.unchecked_into::<Node>());
                }
            },
            VirtualNodeType::DocType => {
                panic!("can not create doctype in document body!");
            }
        }
    }
    
    pub(crate) fn set_element_attributes<DSP>(
        dispatch: &DSP,
        element: &Element,
        attrs: &[&Attribute<MSG>],
        closures: &mut ActiveClosure,
        styles_map: &mut HashMap<String, String>,
        selectors_set: &mut HashSet<String>
    )
    where
        DSP: Dispatch<MSG> + Clone + 'static
    {
        let attrs = merge_attributes_of_same_name(attrs);
        for att in attrs {
            Self::set_element_attribute(dispatch, element, &att, closures, styles_map, selectors_set);
        }
    }

    #[track_caller]
    pub fn set_element_attribute<DSP>(
        dispatch: &DSP,
        element: &Element,
        attr: &Attribute<MSG>,
        closures: &mut ActiveClosure,
        styles_map: &mut HashMap<String, String>,
        selectors_set: &mut HashSet<String>
    )
    where
        DSP: Dispatch<MSG> + Clone + 'static
    {
        let SegregatedAttributes {
            listeners,
            plain_values,
            styles,
            function_calls,
        } =
            partition_callbacks_from_plain_styles_and_func_calls(
                attr,
            );
        
        if let Some(merged_plain_values) =
            merge_plain_attributes_values(&plain_values)
        {
            if let Some(namespace) = attr.namespace() {
                element
                    .set_attribute_ns(
                        Some(namespace),
                        attr.name(),
                        &merged_plain_values,
                    )
                    .unwrap_or_else(|_| {
                        panic!(
                            "Error setting an attribute_ns for {:?}",
                            element
                        )
                    });
            } else {
                match attr.name() {
                    "selectors" => {
                        for s in merged_plain_values.split(";") {
                            let _ = selectors_set.insert(s.to_owned());
                        }
                    },
                    "value" => {
                        Self::set_value_str(element, &merged_plain_values);
                        Self::set_with_values(element, &plain_values);
                    }
                    "open" => {
                        let is_open: bool = plain_values
                            .first()
                            .map(|v| {
                                v.get_simple().map(|v| v.as_bool()).flatten()
                            })
                            .flatten()
                            .unwrap_or(false);
                        Self::set_open(element, is_open);
                    }
                    "checked" => {
                        let is_checked: bool = plain_values
                            .first()
                            .map(|av| {
                                av.get_simple().map(|v| v.as_bool()).flatten()
                            })
                            .flatten()
                            .unwrap_or(false);
                        Self::set_checked(element, is_checked)
                    }
                    "disabled" => {
                        let is_disabled: bool = plain_values
                            .first()
                            .map(|av| {
                                av.get_simple().map(|v| v.as_bool()).flatten()
                            })
                            .flatten()
                            .unwrap_or(false);
                        Self::set_disabled(element, is_disabled);
                    }
                    _ => {
                        element
                            .set_attribute(attr.name(), &merged_plain_values)
                            .unwrap_or_else(|_| {
                                panic!(
                                    "Error setting an attribute for {:?}",
                                    element
                                )
                            });
                    }
                }
            }
        } else if let Some(merged_styles) =
            merge_styles_attributes_values(&styles)
        {
            styles_map.entry(attr.name().to_owned()).or_insert(merged_styles);
        } else {
            element
                .remove_attribute(attr.name())
                .expect("must remove attribute");
        }

        if let Some(merged_func_values) =
            merge_plain_attributes_values(&function_calls)
        {
            if attr.name() == "inner_html" {
                element.set_inner_html(&merged_func_values);
            }
        }

        let mut event_id = 1;
        let unique_id = create_unique_identifier(); 
        for listener in listeners {
            let event_unique_id = unique_id + event_id;
            event_id += 1;

            element
                .set_attribute(DATA_VDOM_ID, &event_unique_id.to_string())
                .expect("Could not set attribute on element");

            closures
                .entry(unique_id)
                .or_insert(vec![]);

            let event_str = attr.name();
            let current_elm: &EventTarget =
                element.dyn_ref().expect("unable to cast to event targe");

            if event_str == "enter" {
                let dispatch_clone = dispatch.clone();
                let listener_clone = listener.clone();
                let key_press_func: Closure<dyn FnMut(web_sys::Event)> =
                    Closure::wrap(Box::new(move |event: web_sys::Event| {
                        let ke: &web_sys::KeyboardEvent = event
                            .dyn_ref()
                            .expect("should be a keyboard event");
                        if ke.key() == "Enter" {
                            let msg = listener_clone.emit(Event::from(event));
                            dispatch_clone.dispatch(msg);
                        }
                    }));

                current_elm
                    .add_event_listener_with_callback(
                        "keypress",
                        key_press_func.as_ref().unchecked_ref(),
                    )
                    .expect("unable to attach enter event listener");
                
                closures
                    .get_mut(&unique_id)
                    .expect("Unable to get closure")
                    .push((event_str, key_press_func));
            } else {
                let callback_wrapped: Closure<dyn FnMut(web_sys::Event)> =
                    create_closure_wrap(dispatch, listener);
                current_elm
                    .add_event_listener_with_callback(
                        event_str,
                        callback_wrapped.as_ref().unchecked_ref(),
                    )
                    .expect("Unable to attached event listener");
                closures
                    .get_mut(&unique_id)
                    .expect("Unable to get closure")
                    .push((event_str, callback_wrapped));
            }
        }
    }

    fn _set_element_focus(element: &Element) {
        let html_element: &HtmlElement = element.unchecked_ref();
        html_element.focus().expect("must focus")
    }

    fn set_checked(element: &Element, is_checked: bool) {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_checked(is_checked);
        } else if let Some(menu_item) = element.dyn_ref::<HtmlMenuItemElement>()
        {
            menu_item.set_checked(is_checked);
        }
    }
    
    fn set_open(element: &Element, is_open: bool) {
        if let Some(details) = element.dyn_ref::<HtmlDetailsElement>() {
            details.set_open(is_open);
        }
    }

    fn set_disabled(element: &Element, is_disabled: bool) {
        if let Some(elm) = element.dyn_ref::<HtmlInputElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlButtonElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlTextAreaElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlStyleElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlLinkElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlSelectElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptionElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptGroupElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlFieldSetElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlMenuItemElement>() {
            elm.set_disabled(is_disabled);
        }
    }

    fn set_value_str(element: &Element, value: &str) {
        if let Some(elm) = element.dyn_ref::<HtmlInputElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlTextAreaElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlSelectElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptionElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlButtonElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlDataElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlOutputElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlParamElement>() {
            elm.set_value(value);
        }
    }

    fn set_value_i32(element: &Element, value: i32) {
        if let Some(elm) = element.dyn_ref::<HtmlLiElement>() {
            elm.set_value(value);
        }
    }

    fn set_value_f64(element: &Element, value: f64) {
        if let Some(elm) = element.dyn_ref::<HtmlMeterElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlProgressElement>() {
            elm.set_value(value);
        }
    }

    fn set_with_values(
        element: &Element,
        values: &[&AttributeValue<MSG>],
    ) {
        let value_i32 = values
            .first()
            .map(|v| v.get_simple().map(|v| v.as_i32()).flatten())
            .flatten();

        let value_f64 = values
            .first()
            .map(|v| v.get_simple().map(|v| v.as_f64()).flatten())
            .flatten();

        if let Some(value_i32) = value_i32 {
            Self::set_value_i32(element, value_i32);
        }
        if let Some(value_f64) = value_f64 {
            Self::set_value_f64(element, value_f64);
        }
    }

    pub fn remove_element_attribute(
        element: &Element,
        attr: &str,
    ) -> Result<(), JsValue> {
        element.remove_attribute(attr)?;

        match attr {
            "value" => {
                Self::set_value_str(element, "");
            }
            "open" => {
                Self::set_open(element, false);
            }
            "checked" => {
                Self::set_checked(element, false);
            }
            "disabled" => {
                Self::set_disabled(element, false);
            }
            _ => (),
        }
        Ok(())
    }
}


pub(crate) fn create_closure_wrap<DSP, MSG>(
    dispatch: &DSP,
    listener: &Listener<MSG>,
) -> Closure<dyn FnMut(web_sys::Event)>
where
        DSP: Dispatch<MSG> + Clone + 'static,
        MSG: Clone + 'static,
{
    let listener_clone = listener.clone();
    let component_clone = dispatch.clone();

    Closure::wrap(Box::new(move |event: web_sys::Event| {
        let is_input = event.type_() == "input";
        let msg = listener_clone.emit(Event::from(event));
        if !is_input {
            component_clone.dispatch(msg);
        }
    }))
}