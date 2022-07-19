use std::any::Any;



#[derive(Debug, Clone)]
pub struct Message {
    pub msg: &'static dyn Any
}


impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        ::core::ptr::eq(&*self.msg, &*other.msg)
    }
}



impl Message {
    pub fn from(msg: &'static dyn Any) -> Self {
        Message {
            msg: msg
        }
    }

    pub fn into<MSG: 'static + Default + Default + Clone + PartialEq>(&self) -> MSG {
        match self.msg.downcast_ref::<MSG>() {
            Some(msg_value) => {
                msg_value.clone()
            }
            None => {
                MSG::default()
            }
        }
    }
}

