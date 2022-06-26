//mod program;
mod duid;
pub mod ports;
pub(crate) mod internal_events;

//pub use program::*;
use ports::PortCmd;
pub use duid::*;

pub type Cmd<APP, MSG> = PortCmd<Duid<APP, MSG>>;
/*
pub type Cmd<APP, MSG> = PortCmd<Program<APP, MSG>>;
pub use window::Window;

/// alias Cmd to use Program as the APP


use wasm_bindgen::prelude::*;
use web_sys::Node;

/// Entry point from Javascript side.
#[wasm_bindgen]
pub fn duid(root: Node) {
    moxie_dom::boot(root, || {
        let (count, incrementer) = state(|| 0);
        let decrementer = incrementer.clone();

        div()
            .child(button().onclick(move |_| decrementer.mutate(|count| *count -= 1)).child("-"))
            .child(count)
            .child(button().onclick(move |_| incrementer.mutate(|count| *count += 1)).child("+"))
    });
}*/