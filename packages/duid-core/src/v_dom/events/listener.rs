//! Callbacks contains function that can be called at a later time.
//! This is used in containing an event listener attached to an DOM element.
use std::any::TypeId;
use std::{fmt, rc::Rc};
use crate::event_manager::Message;


pub struct Listener<IN> {
    /// the function to be executed
    func: Rc<dyn Fn(IN) -> Message>,
    /// the type_id of the function
    func_type_id: TypeId,
    /// the type type_id of the event this callback will be attached to
    event_type_id: TypeId,
    /// the type_id of the return type of this callback when executed.
    msg_type_id: TypeId,
}

impl<IN, F> From<F> for Listener<IN>
where
    F: Fn(IN) -> Message + 'static,
    IN: 'static,
{
    fn from(func: F) -> Self {
        Self {
            func: Rc::new(func),
            func_type_id: TypeId::of::<F>(),
            event_type_id: TypeId::of::<IN>(),
            msg_type_id: TypeId::of::<Message>(),
        }
    }
}


impl<IN> fmt::Debug for Listener<IN> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "in: {:?}, out: {:?}, func: {:?}",
            self.event_type_id, self.msg_type_id, self.func_type_id
        )
    }
}

impl<IN> Listener<IN>
where
    IN: 'static,
{
    /// This method calls the actual callback.
    pub fn emit(&self, input: IN) -> Message {
        (self.func)(input)
    }
}


impl<IN> Clone for Listener<IN> {
    fn clone(&self) -> Self {
        Self {
            func: Rc::clone(&self.func),
            func_type_id: self.func_type_id,
            event_type_id: self.event_type_id,
            msg_type_id: self.msg_type_id,
        }
    }
}


impl<IN> PartialEq for Listener<IN> {
    fn eq(&self, other: &Self) -> bool {
        self.event_type_id == other.event_type_id
            && self.msg_type_id == other.msg_type_id
            && self.func_type_id == other.func_type_id
    }
}