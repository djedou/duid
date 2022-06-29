use crate::{event_manager::Cmd, v_dom::v_node::Node};


pub trait Component<MSG>
where
    MSG: 'static + std::fmt::Debug,
{

    fn update(&mut self, _msg: MSG) -> Cmd<MSG>
    where
        Self: Sized + 'static;

    fn view(&self) -> Node<MSG>;

    fn style(&self) -> String {
        String::new()
    }
}
