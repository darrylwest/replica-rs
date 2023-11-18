use anyhow::Result;
use log::{info, warn};
use serde::Deserialize;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::VERSION;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub home: String,
    pub logging_config: String,
    pub source_folders: Vec<String>,
    pub targets: Vec<String>,
    pub files: Vec<String>,
    pub dbfile: String,
    pub compress: bool,
    pub encrypt: bool,
}

impl Config {
    // read and parse the config file
    pub fn read_config(filename: &str) -> Result<Config> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        let config: Config = toml::from_str(&text).unwrap();

        info!("config: {}, version: {}", config.name, config.version);

        Ok(config)
    }

    /// create and return a copy
    pub fn copy(&self) -> Config {
        Config {
            name: self.name.to_string(),
            version: self.version.to_string(),
            home: self.home.to_string(),
            logging_config: self.logging_config.to_string(),
            source_folders: self.source_folders.clone(),
            targets: self.targets.clone(),
            files: self.files.clone(),
            dbfile: self.dbfile.clone(),
            compress: self.compress,
            encrypt: self.encrypt,
        }
    }

    /// start the logger
    pub fn start_logger(&self) -> Result<()> {
        log4rs::init_file(&self.logging_config, Default::default())?;
        info!("START LOGGER: {}", "-".repeat(80));
        info!("replica version: {}", VERSION);

        Ok(())
    }

    /// write the pid file
    pub fn write_pid_file() {
        let pid = std::process::id().to_string();
        info!("write pid {} to file: {}", pid, crate::PID_FILE);
        let mut file = File::create(crate::PID_FILE).expect("should open the file");
        file.write_all(pid.as_bytes())
            .expect("should write to the pid file")
    }

    /// remove the pid file on exit
    pub fn remove_pid_file() {
        use std::path::Path;
        info!("remove pid dfile: {}", crate::PID_FILE);
        let fp = Path::new(crate::PID_FILE);
        if fp.exists() {
            let resp = std::fs::remove_file(crate::PID_FILE);
            if resp.is_err() {
                warn!("error removing pid: {:?}", resp);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let config = Config::read_config("tests/.replica/config/config.toml").unwrap();
        assert!(!config.name.is_empty());
        assert!(!config.version.is_empty());
        assert!(!config.source_folders.is_empty());
    }

    #[test]
    fn write_remove_pid_file() {
        let pid = std::process::id().to_string();
        Config::write_pid_file();

        let mut file = File::open(crate::PID_FILE).expect("pid file should exist");

        let mut buf = String::new();
        let resp = file.read_to_string(&mut buf);
        assert_eq!(resp.is_ok(), true);
        assert_eq!(buf, pid);

        Config::remove_pid_file();
        let result = File::open(crate::PID_FILE);
        assert_eq!(result.is_err(), true);
    }
}
