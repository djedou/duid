use super::Dispatch;
use std::fmt::Debug;
use super::Cmd;




pub struct PortCmd<DSP> {
    pub commands: Vec<Box<dyn FnOnce(DSP)>>,
    pub(crate) modifier: Modifier,
}

#[derive(Clone)]
pub struct Modifier {
    pub should_update_view: bool,
    pub log_measurements: bool,
    pub measurement_name: String,
}

impl Default for Modifier {
    fn default() -> Self {
        Self {
            should_update_view: true,
            log_measurements: false,
            measurement_name: String::new(),
        }
    }
}

impl<DSP> PortCmd<DSP>
where
    DSP: 'static + Debug,
{
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(DSP) + 'static,
    {
        Self {
            commands: vec![Box::new(f)],
            modifier: Default::default(),
        }
    }

    pub fn batch(cmds: impl IntoIterator<Item = Self>) -> Self {
        let mut commands = vec![];
        let mut should_update_view = false;
        let mut log_measurements = false;
        for cmd in cmds {
            if cmd.modifier.should_update_view {
                should_update_view = true;
            }
            if cmd.modifier.log_measurements {
                log_measurements = true;
            }
            commands.extend(cmd.commands);
        }
        Self {
            commands,
            modifier: Modifier {
                should_update_view,
                log_measurements,
                ..Default::default()
            },
        }
    }

    pub fn append(mut self, cmds: impl IntoIterator<Item = Self>) -> Self {
        for cmd in cmds {
            if cmd.modifier.should_update_view {
                self.modifier.should_update_view = true;
            }
            if cmd.modifier.log_measurements {
                self.modifier.log_measurements = true;
            }
            self.commands.extend(cmd.commands);
        }
        self
    }

    pub fn none() -> Self {
        PortCmd {
            commands: vec![],
            modifier: Default::default(),
        }
    }

    pub fn should_update_view(mut self, should_update_view: bool) -> Self {
        self.modifier.should_update_view = should_update_view;
        self
    }

    pub fn no_render(mut self) -> Self {
        self.modifier.should_update_view = false;
        self
    }
}

impl<DSP> PortCmd<DSP>
where
    DSP: Clone + 'static + Debug,
{
    /// Executes the Cmd
    pub fn emit(self, component: &DSP) {
        for cb in self.commands {
            let component_clone = component.clone();
            cb(component_clone);
        }
    }
}

impl<DSP> PortCmd<DSP> {

    pub fn batch_msg<MSG>(msg_list: Vec<MSG>) -> Self
    where
        MSG: 'static + Debug,
        DSP: Dispatch<MSG> + Clone + 'static + Debug,
    {
        PortCmd::new(move |component: DSP| {
            component.dispatch_multiple(msg_list);
        })
    }
}


impl<DSP, MSG> From<Cmd<MSG>> for PortCmd<DSP>
where
    MSG: 'static + Debug,
    DSP: Dispatch<MSG> + Clone + 'static + Debug,
{
    /// Convert Effects that has only follow ups
    fn from(cmds: Cmd<MSG>) -> Self {
        // we can safely ignore the effects here
        // as there is no content on it.
        let Cmd { messages, modifier } = cmds;
        let mut port_cmd = PortCmd::batch_msg(messages);
        port_cmd.modifier = modifier;
        port_cmd
    }
}
