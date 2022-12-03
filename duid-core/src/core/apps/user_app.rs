use crate::core::{
    html::nodes::Node,
    duid_events::{Cmd, Sub}
};

#[derive(Clone)]
pub struct UserApp<MDL, MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    model: MDL,
    view: fn(&MDL) -> Node<MSG>,
    update: fn(&mut MDL, MSG) -> Cmd<MSG>,
    subscription: fn(&MDL) -> Sub<MSG>
}



impl<MDL, MSG> UserApp<MDL, MSG> 
where 
    MSG: std::fmt::Debug + Clone + 'static,
    MDL: Clone
{

    pub fn new(
        model: MDL,
        view: fn(&MDL) -> Node<MSG>,
        update: fn(&mut MDL, MSG) -> Cmd<MSG>,
        subscription: fn(&MDL) -> Sub<MSG>
    ) -> UserApp<MDL, MSG>  {
        UserApp {
            model,
            view,
            update,
            subscription
        }
    }

    pub fn render(&self) -> Node<MSG> {
        (self.view)(&self.model)
    }

    pub fn update(&mut self, msg: MSG) -> Cmd<MSG> {
        (self.update)(&mut self.model, msg)
    }

    pub fn subscription(&self) -> Sub<MSG> {
        (self.subscription)(&self.model)
    }
}


