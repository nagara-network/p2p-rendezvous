#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod config;
pub mod error;
pub mod handlers;
pub mod protocols;

pub use error::{Error, Result};
pub use nagara_logging::{debug, error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    nagara_logging::init();

    let swarm = config::Config::load().into_swarm()?;
    handlers::event_loop(swarm).await;

    Ok(())
}
