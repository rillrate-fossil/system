use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LOCATION: Location = Location::new("system:proclist");

pub type Pid = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcInfo {
    pub name: String,
    // TODO: Sparklines?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProclistState {
    #[serde(with = "vectorize")]
    procs: BTreeMap<Pid, ProcInfo>,
}

#[allow(clippy::new_without_default)]
impl ProclistState {
    pub fn new() -> Self {
        Self {
            procs: BTreeMap::new(),
        }
    }
}

impl Flow for ProclistState {
    type Action = ProclistAction;
    type Event = ProclistEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate::system::proclist::v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            ProclistEvent::InsertProcess { pid, proc_info } => {
                self.procs.insert(pid, proc_info);
            }
            ProclistEvent::RemoveProcess { pid } => {
                self.procs.remove(&pid);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProclistAction {
    Kill { pid: Pid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProclistEvent {
    InsertProcess { pid: Pid, proc_info: ProcInfo },
    RemoveProcess { pid: Pid },
}
