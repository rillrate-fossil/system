use super::Proclist;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use procfs::process::Stat;
use rillrate_system_protocol::proclist::flow::{Pid, ProcInfo};
use rillrate_system_protocol::proclist::tracer::ProclistTracer;
use std::collections::HashMap;

impl Proclist {
    pub fn spawn_watcher(&mut self, ctx: &mut Context<Self>) {
        let watcher = ProcWatcher::new(self.tracer.clone());
        ctx.spawn_task(watcher, (), ());
    }
}

pub struct ProcWatcher {
    tracer: ProclistTracer,
    snapshot: HashMap<Pid, ProcInfo>,
}

impl ProcWatcher {
    pub fn new(tracer: ProclistTracer) -> Self {
        Self {
            tracer,
            snapshot: HashMap::new(),
        }
    }
}

#[async_trait]
impl LiteTask for ProcWatcher {
    type Output = ();

    async fn repeatable_routine(&mut self) -> Result<Option<Self::Output>, Error> {
        if let Ok(procs) = procfs::process::all_processes() {
            for proc in procs {
                let stat = proc.stat;
                let info = ProcInfo { name: stat.comm };
                //self.snapshot.insert(stat.pid, info);
                self.tracer.insert(stat.pid, info);
            }
        }
        Ok(None)
    }
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
