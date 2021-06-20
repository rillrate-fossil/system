use super::flow::{ProclistState, LOCATION};
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
}
