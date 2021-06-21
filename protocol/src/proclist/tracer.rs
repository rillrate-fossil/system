use super::flow::{Pid, ProcInfo, ProclistEvent, ProclistState, LOCATION};
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::Tracer;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct ProclistTracer {
    tracer: Tracer<ProclistState>,
}

impl ProclistTracer {
    pub fn new() -> Self {
        let state = ProclistState::new();
        let path = LOCATION.root();
        let tracer = Tracer::new_tracer(state, path, None);
        Self { tracer }
    }

    pub fn insert(&self, pid: Pid, proc_info: ProcInfo) {
        let msg = ProclistEvent::InsertProcess { pid, proc_info };
        self.tracer.send(msg, None, None);
    }
}
