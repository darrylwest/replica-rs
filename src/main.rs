//!
//!

use anyhow::Result;
use log::info;
use replica::config::Config;
use replica::file_store::FileStore;
use replica::file_walker::FileWalker;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_config("config/config.toml")?;
    config.start_logger()?;

    info!("replica config: {:?}", config);

    // start the db service (redis?)
    let _filedb = FileStore::new(&config);

    let file_walker = FileWalker::new(config.clone());

    // create the file reader
    let files = file_walker.walk_files()?;
    let mut total_count = files.len();
    for f in files.iter() {
        println!("{}", f.display());
    }


    let files = file_walker.walk_folders()?;
    total_count += files.len();
    for f in files.iter() {
        println!("{}", f.display());
    }

    println!("total count: {}", total_count);

    Ok(())
}
