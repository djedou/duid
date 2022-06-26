/// Util
mod util;
mod window;


pub use util::{
    body, document, history, now, performance, request_animation_frame, window,
};
pub(crate) use util::{request_animation_frame_for_closure};
