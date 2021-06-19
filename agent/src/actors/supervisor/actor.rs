use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, Eliminated, IdOf, InterruptedBy, StartedBy, System};
use rill_engine::{EngineConfig, RillEngine};
use rillrate_system_protocol::provider_type;

pub struct Supervisor {}

impl Supervisor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Actor for Supervisor {
    type GroupBy = ();
}

#[async_trait]
impl StartedBy<System> for Supervisor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let engine = RillEngine::new(EngineConfig::new(provider_type()));
        ctx.spawn_actor(engine, ());
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<System> for Supervisor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}

#[async_trait]
impl Eliminated<RillEngine> for Supervisor {
    async fn handle(
        &mut self,
        _id: IdOf<RillEngine>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        Ok(())
    }
}
