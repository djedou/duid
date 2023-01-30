pub mod core;
pub(crate) mod tailwindcss_system;
pub(crate) mod dom;
pub(crate) mod effects;

pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

