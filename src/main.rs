//!
//!

use anyhow::Result;
use log::info;
use replica::config::Config;
use replica::file_model::FileModel;
use replica::file_walker::FileWalker;
use std::env;

fn main() -> Result<()> {
    let mut home = env::var("HOME").expect("The user should have a home folder.");
    if !home.ends_with("/") {
        home.push('/');
    }
    
    let config = Config::read_config("config/config.toml")?;
    config.start_logger()?;

    info!("replica config: {:?}", config);

    // map with filename as key
    let dbref = FileModel::read_dbfile(&config.dbfile)?;
    let file_walker = FileWalker::new(config.clone());

    // create the file reader
    let mut files = file_walker.walk_folders()?;

    for model in file_walker.walk_files()?.iter() {
        files.push(model.clone());
    }
    println!("total count: {}", files.len());
    for file in files.iter() {
        let p = file.path.to_str().unwrap().replace(home.as_str(), "");
        println!("{}", p);
    }

    FileModel::write_dbfile(&config.dbfile, dbref)?;

    Ok(())
}
