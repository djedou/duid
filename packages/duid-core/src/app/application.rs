use crate::{event_manager::AppCmd, v_dom::v_node::Node};
use std::fmt::Debug;

pub trait Application<MSG>: Debug
where
    MSG: 'static + std::fmt::Debug,
{
    fn init(&mut self) -> AppCmd<Self, MSG>
    where
        Self: Sized + 'static,
    {
        AppCmd::none()
    }
    
    fn update(&mut self, _msg: MSG) -> AppCmd<Self, MSG>
    where
        Self: Sized + 'static;


    fn view(&self) -> Node<MSG>;
    
    fn style(&self) -> String {
        String::new()
    }
}
