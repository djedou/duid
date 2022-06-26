mod component;

use component::Counter;

use duid::{
    Node,
    program::{Duid, Cmd},
    components::Component,
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
    CounterMsg()
}

struct App {
    count: i32,
    children: Vec<Component>
}

impl App {
    fn new() -> Self {
        App { count: 0 }
    }
}

impl Component<Msg> for App {
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
                Counter
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Reset => self.count = 0,
        }
        Cmd::none()
    }
}


#[wasm_bindgen]
pub fn duid(node: &Node) {
    Duid::render(App::new(), &node);
}