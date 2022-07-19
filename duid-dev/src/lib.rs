
use duid::{
    app::{
        DuidApp, Model, Store, Node
    },
    event_manager::Message,
    v_dom::html::{div, text, button},
    v_dom::events::{on_click}
};
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::default::Default;



// Messages
#[derive(PartialEq)]
enum Msg {
    Increment,
    Decrement,
    Reset,
}

#[derive(PartialEq)]
enum ChildMsg {
    Increment,
    Decrement,
    Reset,
}


// Models
#[derive(Debug, Clone, PartialEq)]
struct AppModel {
    count: i32
}


#[derive(Debug, Clone, PartialEq)]
struct ChildModel {
    count: i32
}


impl AppModel {
    fn new() -> Self {
        AppModel {
            count: 10
        }
    }
}


impl ChildModel {
    fn new() -> Self {
        ChildModel {
            count: 20
        }
    }
}

// updates
fn app_update(model: Model, msg: Message) -> Model {
    
    match msg.msg.downcast_ref::<Msg>() {
        Some(app_msg) => {
            match app_msg {
                Msg::Decrement => model,
                Msg::Increment => model,
                Msg::Reset => model
            }
        }
        None => {
            model
        }
    }
}

fn child_update(model: Model, msg: Message) -> Model {
    
    match msg.msg.downcast_ref::<ChildMsg>() {
        Some(app_msg) => {
            match app_msg {
                ChildMsg::Decrement => model,
                ChildMsg::Increment => model,
                ChildMsg::Reset => model
            }
        }
        None => {
            model
        }
    }
}

// Views

fn button_view(label: &'static str) -> Node {

    button(
        Some(Store::new(Model::new(Rc::new(RefCell::new(ChildModel::new()))), child_update)),
        &[on_click(|_| Message::from(&ChildMsg::Decrement))],
        &[text(None, &[], label)]
    )
}

fn app_view() -> Node {
    div(
        Some(Store::new(Model::new(Rc::new(RefCell::new(AppModel::new()))), app_update)),
        &[],
        &[button_view("Djedou Btn")]
    )
}




#[wasm_bindgen]
pub fn duid(node: &str) {
    
    DuidApp::render(app_view(), &node);
}