//!
//!

use anyhow::Result;
use log::info;
use replica::config::Config;
use replica::file_model::FileModel;
use replica::file_walker::FileWalker;

fn main() -> Result<()> {
    let config = Config::read_config("config/config.toml")?;
    config.start_logger()?;

    info!("replica config: {:?}", config);

    // start the db service (redis?)
    // let _filedb = FileStore::new(&config);

    let file_walker = FileWalker::new(config.clone());

    // create the file reader
    let mut files = file_walker.walk_folders()?;

    for model in file_walker.walk_files()?.iter() {
        files.push(model.clone());
    }
    println!("total count: {}", files.len());

    FileModel::write_file(&config.dbfile, files)?;

    Ok(())
}
