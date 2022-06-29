mod dispatch;
mod port;
mod cmd;

use crate::app::DuidApp;

pub use dispatch::*;
pub use port::*;
pub use cmd::*;

pub type AppCmd<APP, MSG> = PortCmd<DuidApp<APP, MSG>>;