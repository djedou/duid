use crate::core::duid_events::{CmdType, Cmd, Dispatch};

#[derive(Debug, Clone)]
pub(crate) struct Effects;
use wasm_bindgen_futures::spawn_local;

impl Effects {
    pub(crate) fn new() -> Self {
        Effects
    }

    pub(crate) fn effects<DSP, MSG>(&self, dispatch: &DSP, cmd: Cmd<MSG>) 
    where 
        DSP: Dispatch<MSG> + Clone + 'static,
        MSG: std::fmt::Debug + Clone + 'static,
    {
        let mut messages = Vec::with_capacity(0);

        for (cmd_type, msg) in cmd.messages {
            match cmd_type {
                CmdType::Normal => {
                    messages.push(msg);
                },
                CmdType::Grpc(grpc_fn) => {
                    let local_dispatch = dispatch.clone();
                    spawn_local(async move {
                        let res = grpc_fn.await;
                        local_dispatch.dispatch_multiple(vec![res]);
                    });
                }
                _ => ()
            }
        }

        if !messages.is_empty() {
            dispatch.dispatch_multiple(messages);
        }
    }
 }