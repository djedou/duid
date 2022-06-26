
/*
#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use std::collections::BTreeMap;
#[cfg(feature = "with-request-animation-frame")]
*/
use crate::program::Cmd;
use crate::program::internal_events::{DomUpdater, Dispatch};
use crate::components::Component;
use crate::dom::document;
use std::any::TypeId;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Node;

/// Holds the user App and the dom updater
/// This is passed into the event listener and the dispatch program
/// will be called after the event is triggered.
pub struct Duid<APP, MSG>
where
    MSG: 'static + std::fmt::Debug,
{
    /// holds the user application
    // Note: This needs to be in Rc<RefCell<_>> to allow interior mutability
    // from a non-mutable reference
    pub app: Rc<RefCell<APP>>,
    
    /// The dom_updater responsible to updating the actual document in the browser
    pub dom_updater: Rc<RefCell<DomUpdater<MSG>>>,
    
}

impl<APP, MSG> Clone for Duid<APP, MSG>
where
    MSG: 'static + std::fmt::Debug,
{
    fn clone(&self) -> Self {
        Duid {
            app: Rc::clone(&self.app),
            dom_updater: Rc::clone(&self.dom_updater),
        }
    }
}

impl<APP, MSG> Duid<APP, MSG>
where
    MSG: 'static + std::fmt::Debug,
    APP: Component<MSG> + 'static,
{
    /// Create an Rc wrapped instance of program, initializing DomUpdater with the initial view
    /// and root node, but doesn't mount it yet.
    pub fn new(
        app: APP,
        mount_node: &Node,
        replace: bool,
        use_shadow: bool,
    ) -> Self {
        let dom_updater: DomUpdater<MSG> = DomUpdater::new(app.view(), mount_node, replace, use_shadow);
        Duid {
            app: Rc::new(RefCell::new(app)),
            dom_updater: Rc::new(RefCell::new(dom_updater)),
        }
    }

    /// executed after the program has been mounted
    fn after_mounted(&self) {
        // call the init of the component
        let cmds = self.app.borrow_mut().init();
        // then emit the cmds, so it starts executing initial calls such (ie: fetching data,
        // listening to events (resize, hashchange)
        cmds.emit(self);

        // inject the style style after call the init of the app as
        // it may be modifying the app state including the style
        let style = self.app.borrow().style();
        if !style.trim().is_empty() {
            let type_id = TypeId::of::<APP>();
            Self::inject_style(type_id, &style);
        }
    }
    /*
    /// get the real DOM node where this app is mounted to.
    pub fn root_node(&self) -> web_sys::Node {
        self.dom_updater.borrow().root_node()
    }

    /// return the node where the app is mounted into
    pub fn mount_node(&self) -> web_sys::Node {
        self.dom_updater.borrow().mount_node()
    }
    */

    pub fn render(app: APP, mount_node: &Node) -> Self {
        console_log::init_with_level(tracing::log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(|info| {
            tracing::error!("{:#?}", info);
        }));
            
        let program = Self::new(app, mount_node, true, false);
        program.mount();
        program
    }
/*

    pub fn append_to_mount(app: APP, mount_node: &Node) -> Self {
        let program = Self::new(app, mount_node, false, false);
        program.mount();
        program
    }

    pub fn mount_to_body(app: APP) -> Self {
        Self::append_to_mount(app, &crate::body())
    }
*/
    /// Do the actual mounting of the view to the specified mount node
    pub fn mount(&self) {
        self.dom_updater.borrow_mut().mount(self);
        self.after_mounted();
    }
/*
    /// explicity update the dom
    pub fn update_dom(&self) {
        let view = self.app.borrow().view();
        // update the last DOM node tree with this new view
        let _total_patches =
            self.dom_updater.borrow_mut().update_dom(self, view);
    }

    /// update the attributes at the mounted element
    pub fn update_mount_attributes(
        &self,
        attributes_value: BTreeMap<String, String>,
    ) {
        let mount_node = self.mount_node();
        let mount_element: &web_sys::Element = mount_node.unchecked_ref();
        for (attr, value) in attributes_value.iter() {
            mount_element
                .set_attribute(attr, value)
                .expect("unable to set attribute in the mount element");
        }
    }
*/
    // Updating stuff start here
    
    /// This is called when an event is triggered in the html DOM.
    /// The sequence of things happening here:
    /// - The app component update is executed.
    /// - The returned Cmd from the component update is then emitted.
    /// - The view is reconstructed with the new state of the app.
    /// - The dom is updated with the newly reconstructed view.
    ///
    ///
    /// TODO: split this function into 2.
    /// - update the app with msgs (use a request_idle_callback)
    /// - compute the view and update the dom (use request_animation_frame )
    fn dispatch_inner(&self, msgs: Vec<MSG>) {
        
        // update the app and emit the cmd returned from the update
        let all_cmd = msgs
            .into_iter()
            .map(|msg| self.app.borrow_mut().update(msg));
        let cmd = Cmd::batch(all_cmd);
        
        if cmd.modifier.should_update_view {
            // a new view is created due to the app update
            //let view = self.app.borrow().view();

            // update the last DOM node tree with this new view
            //let _total_patches =
            //    self.dom_updater.borrow_mut().update_dom(self, view);
        }
        
        cmd.emit(self);
    }

    fn inject_style(type_id: TypeId, style: &str) {
        //use wasm_bindgen::JsCast;
        dbg!(&type_id);
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

/// This will be called when the actual event is triggered.
/// Defined in the DomUpdater::create_closure_wrap function
impl<APP, MSG> Dispatch<MSG> for Duid<APP, MSG>
where
    MSG: 'static + std::fmt::Debug,
    APP: Component<MSG> + 'static,
{
    #[cfg(feature = "with-request-animation-frame")]
    fn dispatch_multiple(&self, msgs: Vec<MSG>) {
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
