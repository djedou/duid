use duid::{
    html::{text, button, nodes::Node},
    events::{on_click},
    duid_events::Cmd
};


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum ButtonMsg {
    Increment,
    Decrement,
    Reset,
}



#[derive(Debug, Clone, PartialEq)]
pub struct ButtonModel {
    pub count: i32,
}


impl ButtonModel {
    pub fn new() -> Self {
        ButtonModel {
            count: 15,
        }
    }
}

// Views
pub fn button_dec_view(_button_model: &ButtonModel) -> Node<ButtonMsg> {

    button(
        &[on_click(|_| ButtonMsg::Decrement)],
        &[
            text("-")
        ]
    )
}

pub fn button_inc_view(_button_model: &ButtonModel) -> Node<ButtonMsg> {

    button(
        &[on_click(|_| ButtonMsg::Increment)],
        &[
            text("+")
        ]
    )
}

// 
pub fn button_update(button_model: &mut ButtonModel, button_msg: ButtonMsg) -> Cmd<ButtonMsg> {
    match button_msg {
        ButtonMsg::Decrement => {
            let value = button_model.count - 1;
            button_model.count = value;
            Cmd::normal(ButtonMsg::Reset)
        },
        ButtonMsg::Increment => {
            let value = button_model.count + 1 ;
            button_model.count = value;
            Cmd::none()
        },
        ButtonMsg::Reset => {
            button_model.count = 0;
            Cmd::none()
        }
    }
}