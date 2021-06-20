use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy};
use rillrate_system_protocol::proclist::tracer::ProclistTracer;

pub struct Proclist {
    tracer: ProclistTracer,
}

impl Proclist {
    pub fn new() -> Self {
        Self {
            tracer: ProclistTracer::new(),
        }
    }
}

impl Actor for Proclist {
    type GroupBy = ();
}

#[async_trait]
impl StartedBy<Supervisor> for Proclist {
    async fn handle(&mut self, _ctx: &mut Context<Self>) -> Result<(), Error> {
        // TODO: Spawn a heartbeat to get a snapshot from procfs
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<Supervisor> for Proclist {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}
