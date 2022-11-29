use crate::config::Config;
use anyhow::Result;
// use crate::file_store::Command;
use log::{debug, info};
// use tokio::sync::{mpsc, oneshot};
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::file_store::FileModel;

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
    pub fn walk_files(&self) -> Result<Vec<FileModel>> {
        info!("walk the folders and files");
        let mut files: Vec<FileModel> = Vec::new();

        for file in self.config.files.iter() {
            let pbuf: PathBuf = [&self.home, file].iter().collect();

            let path = pbuf.as_path();
            if path.is_file() && path.exists() {
                debug!("{}", &pbuf.display());
                let meta = path.metadata()?;
                let modified = meta.modified()?;
                let modified = modified.duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_micros() as u64;
                let len = meta.len();

                let model = FileModel::from(pbuf, len, modified);
                files.push(model);
            }
        }

        Ok(files)
    }

    /// walk all the source folders and gather the files
    pub fn walk_folders(&self) -> Result<Vec<FileModel>> {
        let mut files: Vec<FileModel> = Vec::new();

        for folder in self.config.source_folders.iter() {
            let fname: PathBuf = [&self.home, folder].iter().collect();

            for entry in WalkDir::new(fname).into_iter().filter_map(|e| e.ok()) {
                if entry.file_name() == ".DS_Store" {
                    continue;
                }

                let meta = entry.metadata()?;
                let pbuf = entry.into_path();
                let path = pbuf.as_path();

                if path.is_file() {
                    let modified = meta.modified()?;
                    let modified = modified.duration_since(std::time::SystemTime::UNIX_EPOCH)?;
                    debug!("{} {} {}", &pbuf.display(), meta.len(), modified.as_micros());
                    let model = FileModel::from(pbuf, meta.len(), modified.as_micros() as u64);
                    files.push(model);
                }
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_folders() {
        let config = Config::read_config("tests/config.toml").unwrap();
        let walker = FileWalker::new(config.clone());

        let _ = walker.walk_folders();
    }

    #[test]
    fn walk_files() {
        let config = Config::read_config("tests/config.toml").unwrap();
        let walker = FileWalker::new(config.clone());

        let list = walker.walk_files();
        println!("{:?}", list);
    }
}
