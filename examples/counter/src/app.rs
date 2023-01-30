use duid::{
    html::{div, text, nodes::Node,
        attributes::{classes, selectors}
    },
    duid_events::{NodeMapMsg, Cmd,Sub}
};
use crate::button::{ButtonModel, ButtonMsg, button_dec_view, button_inc_view, button_update};


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    Button(ButtonMsg),
    FromSub(usize)
}

impl From<ButtonMsg> for AppMsg {
    fn from(lhs: ButtonMsg) -> Self {
        AppMsg::Button(lhs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    button_model: ButtonModel
}


impl AppModel {
    pub fn new() -> Self {
        AppModel {
            button_model: ButtonModel::new()
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {
    
    div(
        &[
            classes(&["container".to_owned()]),
            selectors(&[
                "md:::.container:::flex".to_owned()
            ])
        ],
        &[
            button_dec_view(&app_model.button_model).map_msg(|btn_msg| AppMsg::Button(btn_msg)), 
            text(&app_model.button_model.count.to_string()),
            button_inc_view(&app_model.button_model).map_msg(|btn_msg| AppMsg::Button(btn_msg))
        ],
    )
}


pub fn app_update(model: &mut AppModel, msg: AppMsg) -> Cmd<AppMsg> {
    match msg {
        AppMsg::Button(btn_msg) => {
            match btn_msg {
                ButtonMsg::Reset => Cmd::none(),
                _ => button_update(&mut model.button_model, btn_msg).map_cmd_msg()
            }
        },
        AppMsg::FromSub(v) => {
            duid::console::info!("sub value: {v:?}");
            model.button_model.count = v as i32;
            Cmd::none()
        }
    }
}

pub fn app_subscription(model: &AppModel) -> Sub<AppMsg> {
    match model.button_model.count {
        20 => Sub::new(AppMsg::FromSub(25)),
        30 => Sub::new(AppMsg::FromSub(35)),
        40 => Sub::new(AppMsg::FromSub(45)),
        _ => Sub::none()
    }
    
}