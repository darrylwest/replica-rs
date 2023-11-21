//!
//!

use anyhow::Result;
use clap::Parser;
use log::{error, info, warn};
use replica::backup_process::BackupProcess;
use replica::config::Config;
use replica::file_walker::FileWalker;
use replica::kv_store::KeyValueStore;
use std::env;
use std::path::PathBuf;
use std::time::Instant;

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

fn startup(cli: &Cli) -> Config {
    let config = Config::read_config(cli.config.as_str()).expect("config should initialize");
    config.start_logger().expect("logger should start.");

    info!("replica config: {:?}", config);

    config.to_owned()
}

/// the primary process
fn run(cli: Cli, config: Config) -> Result<()> {
    let start_time = Instant::now();

    cd_app_home(config.home.as_str());

    if cli.dryrun {
        warn!("THIS IS A DRY RUN!");
    }

    // read the current database DbOps
    let mut db = KeyValueStore::init(PathBuf::from(config.dbfile.clone()))?;

    let walker = FileWalker::new(config.clone());
    if let Ok(files) = walker.walk_files_and_folders() {
        info!("file count: {}", files.len());

        let target_dir = &config.targets[0];
        let backup = BackupProcess::new(target_dir.as_str(), files.clone(), cli.dryrun);
        let results = backup.process(db.clone());
        if results.is_err() {
            error!("backup failed: {:?}", results);
        } else {
            db = results.unwrap();
            if db.is_dirty() {
                let resp = db.savedb(config.dbfile.as_str());
                if resp.is_err() {
                    error!("database save failed: {:?}", resp);
                }
            }
        }
    }

    let elapsed = (start_time.elapsed().as_nanos() as f64) / 1_000_000_000.0;
    info!("process time: {} seconds", elapsed);
    info!("PROCESS COMPLETE {}", "-".repeat(80));

    Ok(())
}

fn main() -> Result<()> {
    let home = env::var("HOME").expect("The user should have a home folder.");
    cd_app_home(home.as_str());

    let cli = Cli::parse();
    let config = startup(&cli);
    run(cli, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Write};

    fn change_file() {
        let filename = "tests/changed-file.txt";
        let mut buf = File::create(filename).unwrap();
        let msg = format!("the time: {:?}", Instant::now());
        buf.write_all(msg.as_bytes()).unwrap();
    }

    #[test]
    fn startup_test() {
        let test_home = env::current_dir().expect("should get the current working directory");
        let conf_path = format!(
            "{}/.test-replica/config/run-config.toml",
            test_home.to_str().unwrap()
        );

        let cli = Cli {
            config: conf_path,
            verbose: false,
            dryrun: false,
        };

        let config = startup(&cli);
        println!("cli: {:?}", cli);
        println!("ctx: {:?}", config);

        assert!(true);
    }

    #[test]
    fn run_test() {
        change_file();
        let test_home = env::current_dir().expect("should get the current working directory");
        let conf_path = format!(
            "{}/.test-replica/config/run-config.toml",
            test_home.to_str().unwrap()
        );
        let config = Config::read_config(conf_path.as_str()).unwrap();

        println!("conf path : {:?}", conf_path);
        let cli = Cli {
            config: conf_path,
            verbose: false,
            dryrun: false,
        };
        println!("{:?}", cli);
        let results = run(cli, config);
        assert!(results.is_ok());
    }

    #[test]
    fn run_test_dryrun() {
        let test_home = env::current_dir().expect("should get the current working directory");
        let conf_path = format!(
            "{}/.test-replica/config/run-config.toml",
            test_home.to_str().unwrap()
        );
        let config = Config::read_config(conf_path.as_str()).unwrap();
        println!("conf path : {:?}", conf_path);
        let cli = Cli {
            config: conf_path,
            verbose: true,
            dryrun: true,
        };
        println!("{:?}", cli);
        let results = run(cli, config);
        println!("{:?}", results);
        assert!(results.is_ok());
    }

    #[test]
    fn test_app_home() {
        let test_home = env::current_dir().expect("should get the current working directory");
        println!("{}", test_home.display());
        cd_app_home(test_home.to_str().unwrap());
    }

    #[test]
    fn find_exepath() {
        let exepath = env::current_exe().expect("should have an exepath");

        println!("exe path: {}", exepath.display());
    }
}
