use anyhow::Error;
use meio::System;
use rillrate_system::actors::supervisor::Supervisor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    log::info!("RillRate System");
    let sup = Supervisor::new();
    System::spawn_and_wait(sup).await;
    Ok(())
}
