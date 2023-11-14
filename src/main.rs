//!
//!

use anyhow::Result;
use log::info;
use replica::config::Config;
use replica::file_model::FileModel;
use replica::file_walker::FileWalker;
use std::env;

fn main() -> Result<()> {
    let home = env::var("HOME").expect("The user should have a home folder.");

    env::set_current_dir(home.clone()).expect("should be able to change directory to home.");

    let config = Config::read_config(".replica/config/config.toml")?;
    config.start_logger()?;

    info!("replica config: {:?}", config);

    // map with filename as key
    let mut dbref = FileModel::read_dbfile(&config.dbfile)?;
    let file_walker = FileWalker::new(config.clone());

    // create the file reader
    let mut files = file_walker.walk_folders()?;

    for model in file_walker.walk_files()?.iter() {
        files.push(model.clone());
    }

    info!("total count: {}", files.len());
    for file in files.iter() {
        let _ = dbref.insert(file.path.clone(), file.clone());

        let p = file.relative_path();
        info!("{} {} {} {:?} {}", p, file.len, file.modified, file.last_saved, file.hash)
    }

    FileModel::write_dbfile(&config.dbfile, dbref)?;

    Ok(())
}
