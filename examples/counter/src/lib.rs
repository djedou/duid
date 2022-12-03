mod app;
mod button;

use wasm_bindgen::prelude::*;
use duid::apps::{Duid, UserApp};
use app::{AppModel, AppMsg, app_view, app_update, app_subscription};
use std::collections::HashMap;


fn app() -> UserApp<AppModel, AppMsg> {
    UserApp::new(AppModel::new(), app_view, app_update, app_subscription)    
}


#[wasm_bindgen]
pub fn duid(node: &str) {
    
    Duid::start(
        &node, 
        app(),
        HashMap::with_capacity(0), // base_styles
        HashMap::with_capacity(0), // styles
    );
}
