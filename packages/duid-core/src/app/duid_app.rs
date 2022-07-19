use crate::v_dom::{
    v_node::Node,
    Vdom, Vnode, build_vdom
};
use std::rc::Rc;
use std::cell::RefCell;
use crate::dom::RDom;
use indextree::{NodeId};
use crate::event_manager::Dispatcher;


#[derive(Debug)]
pub struct DuidApp
{

    v_dom: Vdom<Vnode>,
    r_dom: Rc<RefCell<RDom>>,
    app_node_id: NodeId,
    dispatcher: Rc<RefCell<Dispatcher>>
    //pub dom_updater: Rc<RefCell<DomUpdater<MSG>>>,

    // ports
    // updater
    // workers
    
}

impl Clone for DuidApp
{
    fn clone(&self) -> Self {
        DuidApp {
            v_dom: self.v_dom.clone(),
            r_dom: Rc::clone(&self.r_dom),
            app_node_id: self.app_node_id.clone(),
            dispatcher: Rc::clone(&self.dispatcher)
        }
    }
}


impl DuidApp
{
    pub fn new<'a>(
        node: Node,
        mount_node: &str,
        replace: bool,
        use_shadow: bool,
    ) -> Self {
        let real_dom: RDom = RDom::new(mount_node, replace, use_shadow);
        let r_dom = Rc::new(RefCell::new(real_dom));
        let (app_node_id, v_dom): (NodeId, Vdom<Vnode>) = build_vdom(node);
        let dispatcher = Dispatcher::new();
        
        DuidApp {
            v_dom,
            r_dom,
            app_node_id,
            dispatcher: Rc::new(RefCell::new(dispatcher))
        }
    }

    /*
    fn after_mounted(&self) {
        let cmds = self.v_dom.borrow_mut().init();

        cmds.emit(self);

        let style = self.v_dom.borrow().style();
        if !style.trim().is_empty() {
            let type_id = TypeId::of::<APP>();
            Self::inject_style(type_id, &style);
        }
    }
*/
    pub fn render(app: Node, mount_node: &str) -> Self {
        console_log::init_with_level(tracing::log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(|info| {
            tracing::error!("{:?}", info);
        }));
        
        //tracing::info!("{:?}", app);
        let program = Self::new(app, mount_node, true, false);
        program.mount();
        program
    }

    pub fn mount(&self) {
        self.r_dom.borrow_mut().mount(self.dispatcher.clone(), &self.v_dom, &self.app_node_id);
        //self.after_mounted();
    }

    /*
    fn _inject_style(type_id: TypeId, style: &str) {
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

    */
}
