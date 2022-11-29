use crate::config::Config;
use anyhow::Result;
// use crate::file_store::Command;
use log::info;
// use tokio::sync::{mpsc, oneshot};
use std::env;
use std::path::PathBuf;

pub struct FileWalker {
    config: Config,
    home: String,
    // req_sender: mpsc::Sender<Command>,
}

impl FileWalker {
    /// create a new FileWalker
    pub fn new(config: Config) -> FileWalker {
        let home = env::var("HOME").unwrap();

        FileWalker { config, home }
    }

    /// walk all the folders and files specified in config source folders and files
    pub fn walk(&self) -> Result<Vec<PathBuf>> {
        info!("walk the folders and files");
        let mut files: Vec<PathBuf> = Vec::new();

        for file in self.config.files.iter() {
            let pbuf: PathBuf = [&self.home, file].iter().collect();
            println!("{}", &pbuf.display());

            let path = pbuf.as_path();
            if path.is_file() && path.exists() {
                files.push(pbuf);
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let config = Config::read_config("tests/config.toml").unwrap();
        let walker = FileWalker::new(config.clone());

        let list = walker.walk();
        println!("{:?}", list);
    }
}
