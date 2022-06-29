#[macro_use]
extern crate doc_comment;


pub mod v_dom;
pub mod dom;
pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

//pub extern crate wasm_bindgen;


pub mod app;
pub mod event_manager;