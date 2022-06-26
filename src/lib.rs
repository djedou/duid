//!
//!  **Duid** is an web framework for creating fast and interactive client side web application.
//!
//!

/*
#![deny(clippy::all)]
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
pub use duid_core::*;

#[cfg(feature = "with-rsx-macro")]
pub use duid_rsx_macro::rsx;


/*
use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub use duid_core::dom;
    pub use duid_core::dom::*;
    pub use duid_core::web_sys;
    pub use duid_core::{components::{Component, Application, Container}, Cmd, program::Program};
    pub use duid_core::js_sys;
    pub use duid_core::mt_dom;
}}

/// reexport prelude from duid core
pub mod prelude {
    pub use duid_core::prelude::*;
    #[cfg(feature = "with-rsx-macro")]
    pub use duid_rsx_macro::rsx;
}
pub use duid_core::{
    html, svg, 
    jss::{jss, jss_ns, units},
    vdom::{ diff, Attribute, Element, Listener, Node, Patch},
    Render,
    components,
    external,
    js,
    ports,
    program,
    vdom
};
#[cfg(feature = "with-rsx-macro")]
pub use duid_rsx_macro::rsx;
*/