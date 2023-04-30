use crate::core::{
    html::{
        a,
        nodes::Node,
        attributes::{classes, selectors, Value, AttributeValue, attr}
    },
    duid_events::NodeMapMsg,
    events::{MouseEvent, on_click},
    util::history,
};
use super::{LinkModel, LinkMsg};
use wasm_bindgen::prelude::JsValue;
use std::rc::Rc;


pub fn link_view<D: Clone + std::fmt::Debug + 'static, M: Clone + 'static>(
    model: &LinkModel<D>, 
    child: Node<M>
) -> Node<LinkMsg<D, M>> {
    let link_classes: Vec<_> = model.classes.iter().map(|c| c.to_owned()).collect();
    let link_selectors: Vec<_> = model.selectors.iter().collect();

    let to = model.get_route();
    let data_clone = Rc::clone(&model.get_link_data());
    let child_node = child.map_msg(|m| LinkMsg::ChildMsg(m));
    
    a(
        &[
            attr("duid-route", AttributeValue::from_value(Value::Bool(true))),
            on_click(move |e: MouseEvent| -> LinkMsg<D, M> {
                e.prevent_default();
                let _ = history().push_state_with_url(&JsValue::NULL, "", Some(to.as_ref()));
                let data = Rc::clone(&data_clone);
                LinkMsg::OnClick(data)
            }),
            classes(&link_classes),
            selectors(&link_selectors),
        ],
        &[
            child_node
        ]
    )
}