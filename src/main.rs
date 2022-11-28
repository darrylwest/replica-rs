//!
//!

use anyhow::Result;
use log::info;
use replica::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/config.toml")?;
    config.start_logger()?;

    info!("replica config: {:?}", config);

    // start the db service (redis?)

    // create the file reader

    Ok(())
}
