//!
//!

use anyhow::Result;
use clap::Parser;
use log::{error, info, warn};
use replica::backup_queue::BackupQueue;
use replica::config::Config;
use replica::file_model::FileModel;
use replica::file_walker::FileWalker;
use std::env;

#[derive(Clone, Debug, Default, Parser)]
#[clap(name = "replica", author, version, about, long_about = None)]
pub struct Cli {
    /// set and alternate configuration file
    #[clap(short, long, value_parser, default_value_t = String::from(".replica/config/config.toml"))]
    pub config: String,

    /// set verbose to log to console
    #[clap(short, long, value_parser)]
    pub verbose: bool,

    /// run the full db read, file walker, queue but skip process queue
    #[clap(short, long, value_parser, default_value_t = true)]
    pub dryrun: bool,
}

/// TODO: refactor this to multiple methods
fn run(cli: Cli) -> Result<()> {
    let config = match Config::read_config(cli.config.as_str()) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("error parsing configuration file: {:?}", cli.config);
            return Err(e);
        }
    };

    config.start_logger()?;

    info!("replica config: {:?}", config);

    let app_home = config.home.as_str();
    let msg = format!("Change to app home: {}", app_home);
    info!("{}", msg.as_str());
    env::set_current_dir(app_home).unwrap_or_else(|_| panic!("{}", msg));

    if cli.dryrun {
        warn!("THIS IS A DRY RUN!");
    }

    // map with filename as key
    let mut dbref = FileModel::read_dbfile(&config.dbfile)?;
    let file_walker = FileWalker::new(config.clone());

    // create the file reader

    let mut files: Vec<FileModel> = Vec::new();
    for model in file_walker.walk_files()?.iter() {
        files.push(model.clone());
    }

    // write the files that were just read
    FileModel::write_dbfile(&config.dbfile, dbref.clone())?;

    let mut queue: Vec<FileModel> = Vec::new();
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
                queue.push(file.clone())
            }
        }
    }

    info!("queue count: {}", queue.len());
    if queue.is_empty() {
        info!("zero files to backup");
        info!("PROCESS COMPLETE {}", "-".repeat(80));
        return Ok(());
    }

    let backup = BackupQueue::new("test", queue, config.dryrun);
    match backup.process() {
        Ok(saved) => {
            info!("update the db reference, len: {}", saved.len());
        }
        Err(e) => {
            error!("backup failed: {e}");
            return Err(e);
        }
    }

    FileModel::write_dbfile(&config.dbfile, dbref)?;

    info!("PROCESS COMPLETE {}", "-".repeat(80));

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
        let home = env::var("HOME").expect("The user should have a home folder.");
        let conf_path = format!("{}{}", home, "/.replica/config/config.toml");
        println!("conf path : {:?}", conf_path);
        let cli = Cli {
            config: conf_path,
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
