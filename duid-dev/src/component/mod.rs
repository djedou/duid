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
enum CounterMsg {
    Increment,
    Decrement,
    Reset,
}

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Component<CounterMsg> for Counter {
    fn view(&self) -> v_node::Node<CounterMsg> {
        rsx! {
            <main>
                <input type="button"
                    value="+"
                    on_click=|_| {
                        CounterMsg::Increment
                    }
                />
                <button class="count" on_click=|_|{CounterMsg::Reset} >{text(self.count)}</button>
                <input type="button"
                    value="-"
                    on_click=|_| {
                        CounterMsg::Decrement
                    }
                />
            </main>
        }
    }

    fn update(&mut self, msg: CounterMsg) -> Cmd<Self, CounterMsg> {
        match msg {
            CounterMsg::Increment => self.count += 1,
            CounterMsg::Decrement => self.count -= 1,
            CounterMsg::Reset => self.count = 0,
        }
        Cmd::none()
    }
}
