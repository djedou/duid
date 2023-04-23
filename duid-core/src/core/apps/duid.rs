use std::rc::Rc;
use std::cell::RefCell;
use crate::{
    core::{
        duid_events::{Dispatch, Cmd},
        util::{window, request_animation_frame_for_closure},
        v_node::ViewBuilder,
        apps::UserApp
    },
    dom::Dom,
    effects::Effects
};
use wasm_bindgen::{closure::Closure};
use wasm_bindgen::JsCast;
use std::collections::HashMap;


pub struct Duid<MDL, MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
    MDL: Clone + 'static,
{
    user_app: Rc<RefCell<UserApp<MDL, MSG>>>,
    dom: Rc<RefCell<Dom<MSG>>>,
    effects: Rc<RefCell<Effects>>
}


impl<MDL, MSG> Clone for Duid<MDL, MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
    MDL: Clone + 'static,
{
    fn clone(&self) -> Self {
        Duid {
            user_app: Rc::clone(&self.user_app),
            dom: Rc::clone(&self.dom),
            effects: Rc::clone(&self.effects),
        }
    }
}


impl<MDL, MSG> Duid<MDL, MSG> 
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
    MDL: Clone + 'static,
{
    pub fn new(
        mount_node: &str,
        virtual_dom: UserApp<MDL, MSG>,
        base_styles: HashMap<String, String>,
        styles: HashMap<String, String>,
        replace: bool,
        use_shadow: bool
    ) -> Self {

        Duid {
            user_app: Rc::new(RefCell::new(virtual_dom)),
            dom: Rc::new(RefCell::new(Dom::new::<Self>(mount_node, replace, use_shadow, base_styles, styles))),
            effects: Rc::new(RefCell::new(Effects::new()))
        }
    }

    pub fn start(
        mount_node: &str,
        virtual_dom: UserApp<MDL, MSG>,
        base_styles: HashMap<String, String>,
        styles: HashMap<String, String>
    ) -> Self
    {
        console_log::init_with_level(tracing::log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(|info| {
            tracing::error!("{:?}", info);
        }));
        
        let program = Self::new(mount_node, virtual_dom, base_styles, styles, true, false);
        
        program.mount();
        program
    }
    
    pub fn mount(&self) {
        let view = self.user_app.borrow().render();
        let mut root_node = ViewBuilder::build(view);
        let _ = root_node.set_key(1);
        self.dom.borrow_mut().mount(self, root_node);
    }

    pub fn render(&self) {
        let view = self.user_app.borrow().render();
        let mut root_node = ViewBuilder::build(view);
        let _ = root_node.set_key(1);
        self.dom.borrow_mut().render(self, root_node);
    }

    pub fn dispatch_inner(&mut self, msgs: Vec<MSG>) {
        let mut cmd_msgs: Vec<_> = msgs.iter().map(|msg| self.user_app.borrow_mut().update(msg.to_owned())).collect();
        let sub_msgs = self.user_app.borrow().subscription();
        cmd_msgs.push(sub_msgs.into());
        let cmd_merged = Cmd::merge_all(cmd_msgs);

        if !cmd_merged.messages.is_empty() {
            self.effects.borrow().effects(self, cmd_merged);
        }
        else {
            self.render();
        }

    }
}


impl<MDL, MSG> Dispatch<MSG> for Duid<MDL, MSG>
where
    MSG: std::fmt::Debug + Clone + PartialEq + 'static,
    MDL: Clone + 'static,
{
    #[cfg(feature = "with-request-animation-frame")]
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
        let mut program_clone = self.clone();
        let closure_raf: Closure<dyn FnMut() + 'static> =
            Closure::once(move || {
                program_clone.dispatch_inner(msgs);
            });
        request_animation_frame_for_closure(&closure_raf);
        closure_raf.forget();
    }

    #[cfg(not(feature = "with-request-animation-frame"))]
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
        self.dispatch_inner(msgs)
    }

    fn dispatch(&self, msg: MSG) {
        self.dispatch_multiple(vec![msg])
    }

    fn dispatch_with_delay(&self, msg: MSG, timeout: i32) -> Option<i32> {
        let program_clone = self.clone();
        let window = window();
        let closure_delay: Closure<dyn FnMut() + 'static> =
            Closure::once(move || {
                program_clone.dispatch(msg);
            });

        let timeout_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure_delay.as_ref().unchecked_ref(),
                timeout,
            )
            .expect("should register the setTimeout call");

        closure_delay.forget();
        Some(timeout_id)
    }
}