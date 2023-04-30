pub mod core;
pub(crate) mod routers;
pub(crate) mod tailwindcss_system;
pub(crate) mod dom;
pub(crate) mod effects;
pub(crate) mod arena;

pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

pub mod web_sys {
    pub use web_sys::{EventTarget, HtmlElement, HtmlStyleElement, Node, CanvasRenderingContext2d, HtmlCanvasElement, Document, Window, History, Location};
}

pub mod duid_routers {
    pub use super::{router, _match_route_regex, match_route_regex, match_route, router_handler};
    pub use super::routers::*;
}