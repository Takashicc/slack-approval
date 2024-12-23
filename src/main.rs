use anyhow::Result;
use tracing::{error, info};

mod services;

fn main() {
    tracing_subscriber::fmt().init();
    info!("Start");
    if let Err(e) = execute() {
        error!("{:?}", e);
        return;
    }
    info!("End")
}

fn execute() -> Result<()> {
    Ok(())
}
