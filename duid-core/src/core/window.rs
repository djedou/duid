use std::fmt::Debug;
use wasm_bindgen::{self, prelude::*, JsCast};

/// Provides access to the Browser window
#[derive(Copy, Clone, Debug)]
pub struct Window;

impl Window {
    pub fn add_event_listeners() {
        let callback_wrapped: Closure<dyn FnMut(web_sys::Event)> = Closure::once(move |_| {
            
        });

        crate::core::util::window()
            .add_event_listener_with_callback(
                "locationchange",
                callback_wrapped.as_ref().unchecked_ref(),
            )
            .expect("Unable to attached event listener");

        callback_wrapped.forget();
    }

    /// set the title of the document
    pub fn set_title(title: &str) {
        super::util::document().set_title(title);
    }

    /// return the size of the browser at this moment
    pub fn get_size() -> (i32, i32) {
        let window = super::util::window();
        let window_width = window
            .inner_width()
            .expect("unable to get window width")
            .as_f64()
            .expect("cant convert to f64");
        let window_height = window
            .inner_height()
            .expect("unable to get height")
            .as_f64()
            .expect("cant convert to f64");
        (window_width as i32, window_height as i32)
    }

    /// return the hash part of the browser current url location
    /// The hash part are the text right after the `#` sign
    pub fn get_hash() -> String {
        super::util::window().location().hash().expect("must have a hash")
    }

    /// set the browser location hash
    pub fn set_location_hash(hash: &str) {
        super::util::window().location().set_hash(hash).expect("must set the location hash");
    }
}
