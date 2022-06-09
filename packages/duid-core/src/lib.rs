#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
//!  Core component of duid
//pub extern crate wasm_bindgen;
#[macro_use]
extern crate doc_comment;

use cfg_if::cfg_if;
cfg_if! {if #[cfg(feature = "with-dom")] {
    /// Dom
    pub mod dom;
    pub use dom::*;
    pub use web_sys;
    pub use js_sys;
    pub use mt_dom;
}}

cfg_if! {if #[cfg(not(feature = "with-dom"))] {
    /// When event is not needed, such as just rendering the dom
    /// tree in server side application
    pub type Event = ();
}}

mod render;
/// Components
pub mod components;
/// External: Internet
pub mod external;
/// Events
pub mod events;
/// JS World
pub mod js;
/// Ports
pub mod ports;
/// Program
pub mod program;

#[macro_use]
pub mod html;
#[macro_use]
pub mod svg;
pub mod vdom;

pub use render::Render;

#[doc(hidden)]
pub use jss;



/// Prelude simplifies the imports from duid
/// This imports the necessary functions to build
/// a basic duid app.
pub mod prelude {
    pub use crate::{
        html::{
            attributes::{attr, *}
        },
        vdom::map_msg::*
    };
    pub use crate::render::Render;
    pub use crate::vdom::map_msg::{AttributeMapMsg, ElementMapMsg, NodeMapMsg};
    pub use crate::vdom::{diff, Attribute, Element, Listener, Node, Patch};
    #[cfg(feature = "with-dom")]
    pub use web_sys;
    #[cfg(feature = "with-debug")]
    pub use log;
}