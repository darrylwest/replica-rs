//!
//!

use anyhow::Result;
use clap::Parser;
use log::{info, warn};
use replica::config::Config;
use replica::file_model::FileModel;
use replica::file_walker::FileWalker;
use std::env;

#[derive(Clone, Debug, Default, Parser)]
#[clap(name = "replica", author, version, about, long_about = None)]
pub struct Cli {
    /// set verbose to log to console
    #[clap(short, long, value_parser)]
    pub verbose: bool,

    /// run the full db read, file walker, queue but skip process queue
    #[clap(short, long, value_parser, default_value_t = true)]
    pub dryrun: bool,
}

fn run(cli: Cli) -> Result<()> {
    let config = Config::read_config(".replica/config/config.toml")?;
    config.start_logger()?;

    info!("replica config: {:?}", config);
    if cli.dryrun {
        warn!("THIS IS A DRY RUN!");
    }

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
        let p = file.relative_path();
        if cli.verbose {
            info!(
                "{} {} {} {:?} {}",
                p, file.len, file.modified, file.last_saved, file.hash
            );
        }

        if let Some(file_ref) = dbref.insert(file.path.clone(), file.clone()) {
            let rmod = file_ref.modified;
            let fmod = file.modified;
            if rmod != fmod {
                info!("QUEUE: {}: {} = {}", p, rmod, fmod);
            }
        }
    }

    FileModel::write_dbfile(&config.dbfile, dbref)?;

    Ok(())
}

fn main() -> Result<()> {
    let home = env::var("HOME").expect("The user should have a home folder.");
    env::set_current_dir(home.clone()).expect("should be able to change directory to home.");

    run(Cli::parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let cli = Cli {
            verbose: true,
            dryrun: true,
        };
        println!("{:?}", cli);
        match run(cli) {
            Ok(()) => {
                assert!(true)
            }
            Err(e) => {
                println!("error: {}", e);
                assert!(false);
            }
        }
    }
}
