
use wasm_bindgen::{closure::Closure, JsCast};

thread_local!(static WINDOW: web_sys::Window = web_sys::window().expect("no global `window` exists"));


pub fn window() -> web_sys::Window {
    WINDOW.with(|window| window.clone())
}

pub fn history() -> web_sys::History {
    window().history().expect("should have a history object")
}

pub fn request_animation_frame<F>(f: F)
where
    F: FnMut() + 'static,
{
    let closure_raf: Closure<dyn FnMut() + 'static> = Closure::once(f);
    window()
        .request_animation_frame(closure_raf.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
    closure_raf.forget();
}

#[allow(unused)]
pub(crate) fn request_animation_frame_for_closure(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

thread_local!(static DOCUMENT: web_sys::Document = window().document().expect("should have a document on window"));

pub fn document() -> web_sys::Document {
    DOCUMENT.with(|document| document.clone())
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("should have performance on window")
}

pub fn now() -> f64 {
    performance().now()
}
