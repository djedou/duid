/*
#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
*/

#[macro_use]
extern crate doc_comment;

pub mod program;
pub mod v_dom;
pub mod dom;
pub use web_sys::Node;
pub mod components;
pub mod svg;
pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

//pub extern crate wasm_bindgen;