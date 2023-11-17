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
    #[clap(short, long, value_parser, default_value_t = false)]
    pub dryrun: bool,
}

/// cd to home folder; panic on fail
fn cd_app_home(app_home: &str) {
    let msg = format!("Change to app home: {}", app_home);
    info!("{}", msg.as_str());
    env::set_current_dir(app_home).unwrap_or_else(|_| panic!("{}", msg));
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
    cd_app_home(config.home.as_str());

    if cli.dryrun {
        warn!("THIS IS A DRY RUN!");
    }

    let walker = FileWalker::new(config.clone());
    if let Ok(files) = walker.walk_files_and_folders() {
        info!("file count: {}", files.len());
        // now compare and update if necessary
        match FileModel::write_dbfile(&config.dbfile, files.clone()) {
            Ok(()) => info!("file model list written to {}", config.dbfile),
            Err(e) => error!("error: {}, writing file model list to {}", e, config.dbfile),
        }

        let target_dir = &config.targets[0];
        let backup = BackupQueue::new(target_dir.as_str(), files, cli.dryrun);
        let results = backup.process();
        if results.is_ok() {
            info!("{} files backed up.", results.unwrap().len());
        } else {
            error!("{:?}", results);
        }
    }

    info!("PROCESS COMPLETE {}", "-".repeat(80));

    Ok(())
}

fn main() -> Result<()> {
    let home = env::var("HOME").expect("The user should have a home folder.");
    cd_app_home(home.as_str());

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

    #[test]
    fn test_app_home() {
        let home = env::var("HOME").expect("The user should have a home folder.");
        cd_app_home(home.as_str());
    }
}
