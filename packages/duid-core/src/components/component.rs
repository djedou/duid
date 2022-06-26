use crate::{program::Cmd, v_dom::v_node::Node};

/// An Application is the root component of your program.
/// Everything that happens in your application is done here.
///
pub trait Component<MSG>
where
    MSG: 'static + std::fmt::Debug,
{
    ///  The application can implement this method where it can modify its initial state.
    ///  This method is called right after the program is mounted into the DOM.
    fn init(&mut self) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static,
    {
        Cmd::none()
    }

    /// Update the component with a message.
    /// The update function returns a Cmd, which can be executed by the runtime.
    ///
    /// Called each time an action is triggered from the view
    fn update(&mut self, _msg: MSG) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static;

    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;

    /// optionally an Application can specify its own css style
    fn style(&self) -> String {
        String::new()
    }
}
