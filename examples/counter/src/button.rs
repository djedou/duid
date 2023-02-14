use duid::{
    html::{text, button, nodes::Node,
        attributes::{classes, selectors}
    },
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
        &[
            classes(&["bg-btn".to_owned()]),
            selectors(&[
                "md:::.bg-btn:::flex m-2 p-6 bg-gradient-to-t from-green-600 via-violet-600 outline outline-offset-2 outline-2 outline-blue-500".to_owned(),
            ]),
            on_click(|_| ButtonMsg::Decrement)
        ],
        &[
            text("-")
        ]
    )
}

pub fn button_inc_view(_button_model: &ButtonModel) -> Node<ButtonMsg> {

    button(
        &[
            classes(&["btn".to_owned()]),
            selectors(&[
                "div > *:::block".to_owned(), 
                ".btn-item".to_owned(),
                "lg:::.btn:::flex bg-color-green-600".to_owned(),
                "md:::.btn:::flex m-2 p-6 bg-color-rose-600".to_owned(),
                ".btn:::flex bg-color-yellow-200 font-medium".to_owned(),
                "md:::.btn:hover:::bg-color-green-200".to_owned()
            ]),
            on_click(|_| ButtonMsg::Increment)
        ],
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