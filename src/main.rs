//!
//!

use anyhow::Result;
use log::info;
use replica::config::Config;
use replica::file_store::FileStore;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/config.toml")?;
    config.start_logger()?;

    info!("replica config: {:?}", config);

    // start the db service (redis?)
    let _filedb = FileStore::new(&config);

    // create the file reader

    Ok(())
}
