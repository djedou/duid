mod util;
mod window;
mod r_dom;

pub(crate) use r_dom::*;


pub use util::{
    body, document, history, now, performance, request_animation_frame, window,
};
pub(crate) use util::{request_animation_frame_for_closure};
