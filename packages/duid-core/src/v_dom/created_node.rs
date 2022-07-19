use crate::{
    dom::{document},
    v_dom::{
        v_node::{NodeType, LeafType},
        Vdom, Vnode,
        events::{Event, Listener}
    }
};
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use crate::event_manager::Dispatcher;
use indextree::{NodeId};
use std::collections::HashMap;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    self, HtmlInputElement,EventTarget, Element, HtmlButtonElement, HtmlDataElement,
    HtmlDetailsElement, HtmlElement, HtmlFieldSetElement,
    HtmlLiElement, HtmlLinkElement, HtmlMenuItemElement, HtmlMeterElement,
    HtmlOptGroupElement, HtmlOptionElement, HtmlOutputElement,
    HtmlParamElement, HtmlProgressElement, HtmlSelectElement, HtmlStyleElement,
    HtmlTextAreaElement, Node, Document
};
use crate::v_dom::html::attributes::{
    Attribute, merge_attributes_of_same_name, SegregatedAttributes, partition_callbacks_from_plain_styles_and_func_calls,
    merge_plain_attributes_values, merge_styles_attributes_values, AttributeValue
};




thread_local!(static NODE_ID_COUNTER: Cell<usize> = Cell::new(1));

/// This is the value of the data-sauron-vdom-id.
/// Used to uniquely identify elements that contain closures so that the DomUpdater can
/// look them up by their unique id.
fn create_unique_identifier() -> usize {
    let id = NODE_ID_COUNTER.with(|x| {
        let tmp = x.get();
        x.set(tmp + 1);
        tmp
    });
    id
}

pub(crate) const DATA_VDOM_ID: &str = "duid-event-id";

/// Closures that we are holding on to to make sure that they don't get invalidated after a
/// VirtualNode is dropped.
///
/// The u32 is a unique identifier that is associated with the DOM element that this closure is
/// attached to.
///
pub type ActiveClosure = HashMap<usize, Vec<(&'static str, Closure<dyn FnMut(web_sys::Event)>)>>;



#[derive(Debug)]
pub struct CreatedNode {
    /// A `Node` or `Element` that was created from a `Vnode`
    pub node: Node
}

impl CreatedNode {


    /// Create and return a `CreatedNode` instance for this virtual node.
    pub fn create_dom_node(duid: Rc<RefCell<Dispatcher>>, vdom: &Vdom<Vnode>, app_node_id: &NodeId) -> CreatedNode {
        let doc = document();
        //tracing::info!("vdom: {:?}", vdom);
        let app = vdom.get(app_node_id.clone());
        let app_node: Node = Self::create_doc_node(&app.unwrap().get(), &doc, duid.clone());
        
        let _ = app_node_id.children(&vdom).into_iter().map(|child_id| {Self::create_children(&vdom, &child_id, &app_node, &doc, duid.clone());}).collect::<Vec<_>>();
        CreatedNode { node: app_node }
    }

    fn create_doc_node(vnode: &Vnode, doc: &Document, duid: Rc<RefCell<Dispatcher>>) -> Node {
        let element = if let Some(namespace) = vnode.namespace.clone() {
            doc
                .create_element_ns(Some(&namespace), &vnode.tag)
                .expect("Unable to create element")
        } else {
            doc
                .create_element(&vnode.tag)
                .expect("Unable to create element")
        };
        
        let attrs = vnode.props.iter().map(|attr| attr).collect::<Vec<_>>();
        Self::set_element_attributes(duid.clone(), &element, &attrs);
        
        element.unchecked_into()
    }

    fn create_children(vdom: &Vdom<Vnode>, v_node_id: &NodeId, parent_node: &Node, doc: &Document, duid: Rc<RefCell<Dispatcher>>) {
        // 1. Get the Vnode it self.
        if let Some(vnode) = vdom.get(v_node_id.clone()) {
            let node = vnode.get();
            // 2. Check its type (Element ?, Fragment ?, Leaf ?).
            match &node.node_type {
                NodeType::Element => {
                    let child_node: Node = Self::create_doc_node(&node, &doc, duid.clone());
                    let _ = v_node_id.children(&vdom).into_iter().map(|child_id| {Self::create_children(&vdom, &child_id, &child_node, &doc, duid.clone());}).collect::<Vec<_>>();
                    let _ = parent_node.append_child(&child_node);
                },
                NodeType::Fragment => {
                    let doc_fragment: Node = doc.create_document_fragment().unchecked_into();
                    let _ = v_node_id.children(&vdom).into_iter().map(|child_id| {Self::create_children(&vdom, &child_id, &doc_fragment, &doc, duid.clone());}).collect::<Vec<_>>();
                    let _ = parent_node.append_child(&doc_fragment);
                },
                NodeType::Leaf(leaf) => {
                    match leaf {
                        LeafType::Text => {
                            let text = match &node.leaf_value {
                                Some(v) => v.clone(),
                                None => ""
                            };
                            let attrs = node.props.iter().map(|attr| attr).collect::<Vec<_>>();
                            let text_node: Element = doc.create_text_node(&text).unchecked_into();
                            Self::set_element_attributes(duid.clone(), &text_node, &attrs);
                            let _ = parent_node.append_child(&text_node);
                        },
                        LeafType::Comment => {
                            let text = match &node.leaf_value {
                                Some(v) => v.clone(),
                                None => ""
                            };
                            
                            let comment_node: Node = doc.create_comment(&text).unchecked_into();
                            let _ = parent_node.append_child(&comment_node);
                        },
                        LeafType::DocType => {}
                    }
                }
            }
        }
    }
    
    /*
    /// dispatch the mount event,
    /// call the listener since browser don't allow asynchronous execution of
    /// dispatching custom events (non-native browser events)
    fn dispatch_mount_event<DSP, MSG>(
        component: &DSP,
        velem: &v_node::Element<MSG>,
        element: &Element,
    ) where
        MSG: 'static + std::fmt::Debug,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        for att in velem.attrs.iter() {
            if *att.name() == "mount" {
                for val in att.value().iter() {
                    if let AttributeValue::EventListener(cb) = val {
                        let msg = cb.emit(Event::from(MountEvent {
                            target_node: element.clone().unchecked_into(),
                        }));
                        component.dispatch(msg);
                    }
                }
            }
        }
    }
*/

    /// set the element attribute
    pub fn set_element_attributes(
        dispatcher: Rc<RefCell<Dispatcher>>,
        element: &Element,
        attrs: &[&Attribute],
    )
    {
        let attrs = merge_attributes_of_same_name(attrs);
        for att in attrs {
            Self::set_element_attribute(dispatcher.clone(), element, &att);
        }
    }

    /// set the element attribute
    ///
    /// Note: this is called in a loop, so setting the attributes, and style will not be on
    /// the same call, but on a subsequent call to each other. Using the if-else-if here for
    /// attributes, style, function_call.
    #[track_caller]
    pub fn set_element_attribute(
        dispatcher: Rc<RefCell<Dispatcher>>,
        element: &Element,
        attr: &Attribute,
    )
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

        // set simple values
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
            // set the styles
            element
                .set_attribute(attr.name(), &merged_styles)
                .unwrap_or_else(|_| {
                    panic!("Error setting an attribute_ns for {:?}", element)
                });
        } else {
            //if the merged attribute is blank of empty when string is trimmed
            //remove the attribute
            element
                .remove_attribute(attr.name())
                .expect("must remove attribute");
        }

        // do function calls such as set_inner_html
        if let Some(merged_func_values) =
            merge_plain_attributes_values(&function_calls)
        {
            if attr.name() == "inner_html" {
                element.set_inner_html(&merged_func_values);
            }
        }

        // add listeners using add_event_listener
        for listener in listeners {
            let unique_id = create_unique_identifier();

            // set the data-sauron_vdom-id this will be read later on
            // when it's time to remove this element and its closures and event listeners
            element
                .set_attribute(DATA_VDOM_ID, &unique_id.to_string())
                .expect("Could not set attribute on element");

            dispatcher.borrow_mut().active_closures.borrow_mut().insert(unique_id, vec![]);

            let event_str = attr.name();
            let current_elm: &EventTarget =
                element.dyn_ref().expect("unable to cast to event targe");

            // a custom enter event which triggers the listener
            // when the enter key is pressed
            if event_str == "enter" {
                let dispatcher_clone = dispatcher.clone();
                let listener_clone = listener.clone();
                let key_press_func: Closure<dyn FnMut(web_sys::Event)> =
                    Closure::wrap(Box::new(move |event: web_sys::Event| {
                        let ke: &web_sys::KeyboardEvent = event
                            .dyn_ref()
                            .expect("should be a keyboard event");
                        if ke.key() == "Enter" {
                            let msg = listener_clone.emit(Event::from(event));
                            dispatcher_clone.borrow().dispatch(msg);
                        }
                    }));

                current_elm
                    .add_event_listener_with_callback(
                        "keypress",
                        key_press_func.as_ref().unchecked_ref(),
                    )
                    .expect("unable to attach enter event listener");

                key_press_func.forget();
            } else {
                // This is where all of the UI events is wired in this part of the code.
                // All event listener is added to this element.
                // The callback to this listener emits an Msg which is then \
                // dispatched to the `component` which then triggers update view cycle.
                let callback_wrapped: Closure<dyn FnMut(web_sys::Event)> =
                    create_closure_wrap(dispatcher.clone(), listener);
                current_elm
                    .add_event_listener_with_callback(
                        event_str,
                        callback_wrapped.as_ref().unchecked_ref(),
                    )
                    .expect("Unable to attached event listener");
                dispatcher.borrow_mut().active_closures.borrow_mut()
                    .get_mut(&unique_id)
                    .expect("Unable to get closure")
                    .push((event_str, callback_wrapped));
            }
        }
    }

    /// set focus to this element
    pub(crate) fn set_element_focus(element: &Element) {
        let html_element: &HtmlElement = element.unchecked_ref();
        html_element.focus().expect("must focus")
    }

    /// explicitly call `set_checked` function on the html element
    /// since setting the attribute to false will not unchecked it.
    ///
    /// There are only 2 elements where set_checked is applicable:
    /// - input
    /// - menuitem
    fn set_checked(element: &Element, is_checked: bool) {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_checked(is_checked);
        } else if let Some(menu_item) = element.dyn_ref::<HtmlMenuItemElement>()
        {
            menu_item.set_checked(is_checked);
        }
    }

    /// explicitly call set_open for details
    /// since setting the attribute `open` to false will not close it.
    ///
    /// TODO: HtmlDialogElement ( but it is not supported on firefox and in safarit, only works on chrome)
    ///
    /// Applies to:
    ///  - dialog
    ///  - details
    fn set_open(element: &Element, is_open: bool) {
        if let Some(details) = element.dyn_ref::<HtmlDetailsElement>() {
            details.set_open(is_open);
        }
    }

    /// explicitly call on `set_disabled`
    /// since setting the attribute `disabled` false will not enable it.
    ///
    /// These are 10 elements that we can call `set_disabled` function to.
    /// - input
    /// - button
    /// - textarea
    /// - style
    /// - link
    /// - select
    /// - option
    /// - optgroup
    /// - fieldset
    /// - menuitem
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

    /// we explicitly call the `set_value` function in the html element
    ///
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

    /// set the element attribute value with the first numerical value found in values
    fn set_with_values(
        element: &Element,
        values: &[&AttributeValue],
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

    /// remove element attribute,
    /// takes care of special case such as checked
    pub fn remove_element_attribute(
        element: &Element,
        attr: &Attribute,
    ) -> Result<(), JsValue> {
        //log::trace!("removing attribute: {}", attr.name());

        element.remove_attribute(attr.name())?;

        match attr.name() {
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


/// This wrap into a closure the function that is dispatched when the event is triggered.
pub(crate) fn create_closure_wrap(
    dispatcher: Rc<RefCell<Dispatcher>>,
    listener: &Listener,
) -> Closure<dyn FnMut(web_sys::Event)>
{
    let listener_clone = listener.clone();
    let component_clone = dispatcher.clone();

    Closure::wrap(Box::new(move |event: web_sys::Event| {
        let msg = listener_clone.emit(Event::from(event));
        component_clone.borrow().dispatch(msg);
    }))
}
