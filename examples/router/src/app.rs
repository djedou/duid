use duid::{
    html::{div, text, nodes::Node,
        attributes::{classes, selectors}
    },
    duid_events::{NodeMapMsg, Cmd, Sub},
    duid_routers::{Params, router, link_view, LinkModel, LinkMsg, router_handler},
};
use crate::button::{ButtonModel, ButtonMsg, button_dec_view, button_inc_view, button_update};

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    Button(ButtonMsg),
    FromSub(usize),
    Root(AppRootMsg),
    User(AppUserMsg),
}

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppRootMsg {
    Link(LinkMsg<(), ChildMsg>)
}

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppUserMsg {
    Link(LinkMsg<(), ChildMsg>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChildMsg {
    NoAction
}


impl From<LinkMsg<(), ChildMsg>> for AppUserMsg {
    fn from(lhs: LinkMsg<(), ChildMsg>) -> Self {
        AppUserMsg::Link(lhs)
    }
}

impl From<LinkMsg<(), ChildMsg>> for AppRootMsg {
    fn from(lhs: LinkMsg<(), ChildMsg>) -> Self {
        AppRootMsg::Link(lhs)
    }
}

impl From<AppRootMsg> for AppMsg {
    fn from(lhs: AppRootMsg) -> Self {
        AppMsg::Root(lhs)
    }
}

impl From<AppUserMsg> for AppMsg {
    fn from(lhs: AppUserMsg) -> Self {
        AppMsg::User(lhs)
    }
}

impl From<ButtonMsg> for AppMsg {
    fn from(lhs: ButtonMsg) -> Self {
        AppMsg::Button(lhs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    button_model: ButtonModel,
    link_root_model: LinkModel<()>,
    link_user_model: LinkModel<()>,
    route: String
}


impl AppModel {
    pub fn new() -> Self {
        let mut link_root_model = LinkModel::new();
        link_root_model.set_route("/user");
        let mut link_user_model = LinkModel::new();
        link_user_model.set_route("/");

        AppModel {
            button_model: ButtonModel::new(),
            link_root_model,
            link_user_model,
            route: String::from("/")
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    match router!(AppModel, AppMsg, app_model.to_owned(), app_model.route.as_ref(), {
        "/" => {
            router_handler!(AppModel, AppMsg, app_view_root)
        },
        "/user" => {
            router_handler!(AppModel, AppMsg, app_view_user)
        }
    }) {
        Ok(node) => node,
        Err(_) => div(&[],&[])
    }
}

pub fn app_view_root(app_model: AppModel, _params: Params) -> Node<AppMsg> {
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
            button_inc_view(&app_model.button_model).map_msg(|btn_msg| AppMsg::Button(btn_msg)),
            link_view(&app_model.link_root_model, text("to user")).map_msg(|lk_msg| AppMsg::from(AppRootMsg::from(lk_msg)))
        ],
    )
}

pub fn app_view_user(app_model: AppModel, _params: Params) -> Node<AppMsg> {
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
            button_inc_view(&app_model.button_model).map_msg(|btn_msg| AppMsg::Button(btn_msg)),
            link_view(&app_model.link_user_model, text("to root")).map_msg(|lk_msg| AppMsg::from(AppUserMsg::from(lk_msg))),
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
            model.button_model.count = v as i32;
            Cmd::none()
        },
        AppMsg::Root(AppRootMsg::Link(_)) => {
            model.route = model.link_root_model.get_route();
            Cmd::none()
        },
        AppMsg::User(AppUserMsg::Link(_)) => {
            model.route = model.link_user_model.get_route();
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