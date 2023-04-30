pub mod apps;
pub mod duid_events;
pub mod events;
pub mod html;
pub mod svg;
pub(crate) mod v_node;
pub mod util;
pub mod networks;
pub mod window;

use std::cell::Cell;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;


thread_local!(static NODE_ID_COUNTER: Cell<usize> = Cell::new(1));


pub(crate) fn create_unique_identifier() -> usize {
    let id = NODE_ID_COUNTER.with(|x| {
        let tmp = x.get();
        x.set(tmp + 1);
        tmp
    });
    id
}

/*
pub(crate) fn clean_unique_identifier() {
    NODE_ID_COUNTER.with(|x| {
        x.set(1);
    });
}


pub(crate) fn set_unique_identifier(id: usize) {
    NODE_ID_COUNTER.with(|x| {
        x.set(id);
    });
}
*/

pub(crate) type ActiveClosure = HashMap<usize, Vec<(&'static str, Closure<dyn FnMut(web_sys::Event)>)>>;
pub(crate) const DATA_VDOM_ID: &str = "dom-event-id";