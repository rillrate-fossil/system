use super::Proclist;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use rillrate_system_protocol::proclist::tracer::ProclistTracer;

impl Proclist {
    pub fn spawn_watcher(&mut self, ctx: &mut Context<Self>) {
        let watcher = ProcWatcher::new(self.tracer.clone());
        ctx.spawn_task(watcher, (), ());
    }
}

pub struct ProcWatcher {
    tracer: ProclistTracer,
}

impl ProcWatcher {
    pub fn new(tracer: ProclistTracer) -> Self {
        Self { tracer }
    }
}

impl LiteTask for ProcWatcher {
    type Output = ();
}

#[async_trait]
impl TaskEliminated<ProcWatcher, ()> for Proclist {
    async fn handle(
        &mut self,
        _id: IdOf<ProcWatcher>,
        _tag: (),
        _res: Result<(), TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        Ok(())
    }
}
