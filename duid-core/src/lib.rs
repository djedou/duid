pub mod core;
pub(crate) mod tailwindcss_system;
pub(crate) mod dom;
pub(crate) mod effects;

pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

pub mod web_sys {
    pub use web_sys::{EventTarget, HtmlElement, HtmlStyleElement, Node, CanvasRenderingContext2d, HtmlCanvasElement, Document, Window};
}