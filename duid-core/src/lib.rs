pub mod core;
pub(crate) mod tailwindcss_system;
pub(crate) mod dom;
pub(crate) mod effects;
pub(crate) mod arena;

pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

pub mod web_sys {
    pub use web_sys::{EventTarget, HtmlElement, HtmlStyleElement, Node, CanvasRenderingContext2d, HtmlCanvasElement, Document, Window};
}

pub mod duid_nuts {
    pub use nuts::publish;

    #[derive(Debug)]
    pub struct NutsActivityMsg {
        pub msg: Box<dyn std::fmt::Debug>
    }

    impl NutsActivityMsg {
        pub fn new<MSG: std::fmt::Debug + 'static>(msg: MSG) -> NutsActivityMsg {
            NutsActivityMsg {
                msg: Box::new(msg)
            }
        }
    }
}