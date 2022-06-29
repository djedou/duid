use crate::event_manager::PortCmd;
use crate::event_manager::Dispatch;
use super::Application;
use crate::dom::{document, RDom};
use std::any::TypeId;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
//use web_sys::Node;
use std::marker::PhantomData;
use std::fmt::Debug;


#[derive(Debug)]
pub struct DuidApp<APP, MSG>
where
    MSG: 'static + Debug,
{

    v_dom: Rc<RefCell<APP>>,
    r_dom: Rc<RefCell<RDom>>,
    //pub dom_updater: Rc<RefCell<DomUpdater<MSG>>>,

    // ports
    // updater
    phantom: PhantomData<MSG>
    
}

impl<APP, MSG> Clone for DuidApp<APP, MSG>
where
    MSG: 'static + Debug,
{
    fn clone(&self) -> Self {
        DuidApp {
            v_dom: Rc::clone(&self.v_dom),
            r_dom: Rc::clone(&self.r_dom),
            phantom: PhantomData,
        }
    }
}

impl<APP, MSG> DuidApp<APP, MSG>
where
    MSG: 'static + Debug,
    APP: Application<MSG> + 'static + Debug,
{
    pub fn new(
        app: APP,
        mount_node: &str,
        replace: bool,
        use_shadow: bool,
    ) -> Self {
        let real_dom: RDom = RDom::new(mount_node, replace, use_shadow);
        DuidApp {
            v_dom: Rc::new(RefCell::new(app)),
            r_dom: Rc::new(RefCell::new(real_dom)),
            phantom: PhantomData
        }
    }


    fn after_mounted(&self) {
        let cmds = self.v_dom.borrow_mut().init();

        cmds.emit(self);

        let style = self.v_dom.borrow().style();
        if !style.trim().is_empty() {
            let type_id = TypeId::of::<APP>();
            Self::inject_style(type_id, &style);
        }
    }

    pub fn render(app: APP, mount_node: &str) -> Self {
        console_log::init_with_level(tracing::log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(|info| {
            tracing::error!("{:#?}", info);
        }));
            
        let program = Self::new(app, mount_node, true, false);
        program.mount();
        program
    }

    pub fn mount(&self) {
        self.r_dom.borrow_mut().mount(self, &self.v_dom.borrow().view());
        self.after_mounted();
    }

    fn dispatch_inner(&self, msgs: Vec<MSG>) {

        let all_cmd = msgs
            .into_iter()
            .map(|msg| self.v_dom.borrow_mut().update(msg));
        let cmd = PortCmd::batch(all_cmd);

        cmd.emit(self);
    }

    fn inject_style(type_id: TypeId, style: &str) {
        //dbg!(&type_id);
        let type_id = format!("{:?}", type_id);

        let document = document();
        let html_style = document
            .create_element("style")
            .expect("must be able to create style element");
        html_style
            .set_attribute("class", &type_id)
            .expect("must set attribute");
        let html_style: web_sys::Node = html_style.unchecked_into();
        html_style.set_text_content(Some(style));
        let head = document.head().expect("must have a head");
        head.append_child(&html_style).expect("must append style");
    }
}

impl<APP, MSG> Dispatch<MSG> for DuidApp<APP, MSG>
where
    MSG: 'static + Debug,
    APP: Application<MSG> + 'static + Debug,
{
    #[cfg(feature = "with-request-animation-frame")]
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
        crate::console::info!("clicked");
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

    fn dispatch(&self, msg: MSG) {
        self.dispatch_multiple(vec![msg])
    }
}
