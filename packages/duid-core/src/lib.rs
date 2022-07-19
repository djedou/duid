pub mod v_dom;
pub mod dom;
pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}


pub mod app;
pub mod event_manager;