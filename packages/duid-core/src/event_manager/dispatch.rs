use crate::event_manager::Message;
use wasm_bindgen::{closure::Closure};
use crate::v_dom::ActiveClosure;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;




#[derive(Debug)]
pub struct Dispatcher {
    pub active_closures: Rc<RefCell<ActiveClosure>> 
}

impl Clone for Dispatcher {
    fn clone(&self) -> Self {
        Dispatcher {
            active_closures: self.active_closures.clone()
        }
    }
}

impl Dispatcher {

    pub fn new() -> Self {
        Dispatcher {
            active_closures: Rc::new(RefCell::new(HashMap::new()))
        }
    } 

    pub fn dispatch_inner(&self, msgs: Vec<Message>) {
        
        crate::console::info!("msgs: {:?}", msgs);
        
    }

    #[cfg(feature = "with-request-animation-frame")]
    fn dispatch_multiple(&self, msgs: Vec<Message>) {
        let program_clone = self.clone();
        let closure_raf: Closure<dyn FnMut() + 'static> =
            Closure::once(move || {
                program_clone.dispatch_inner(msgs);
            });
        crate::dom::request_animation_frame_for_closure(&closure_raf);
        closure_raf.forget();
    }

    #[cfg(not(feature = "with-request-animation-frame"))]
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
        self.dispatch_inner(msgs)
    }

    pub fn dispatch(&self, msg: Message) {
        self.dispatch_multiple(vec![msg])
    }
}
