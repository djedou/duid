
use duid::{
    app::{DuidApp, Application},
    event_manager::AppCmd,
    rsx,
    v_dom::v_node,
    v_dom::html::{text},
};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
enum Msg {
    Increment,
    Decrement,
    Reset,
}

#[derive(Debug)]
struct App {
    count: i32,
}

impl App {
    fn new() -> Self {
        App { count: 0 }
    }
}

impl Application<Msg> for App {
    fn view(&self) -> v_node::Node<Msg> {
        rsx! {
            <main>
                <input type="button"
                    value="+"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <button class="count" on_click=|_|{Msg::Reset} >{text(self.count)}</button>
                <input type="button"
                    value="-"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> AppCmd<Self, Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Reset => self.count = 0,
        }
        AppCmd::none()
    }
}


#[wasm_bindgen]
pub fn duid(node: &str) {
    DuidApp::render(App::new(), &node);
}