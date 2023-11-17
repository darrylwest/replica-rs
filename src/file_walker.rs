use crate::config::Config;
use crate::file_model::FileModel;
use anyhow::Result;
use log::{debug, error, info};
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct FileWalker {
    config: Config,
    home: String,
}

impl FileWalker {
    /// create a new FileWalker
    pub fn new(config: Config) -> FileWalker {
        let home = env::var("HOME").unwrap();

        FileWalker { config, home }
    }

    /// walk the files and folders
    pub fn walk_files_and_folders(&self) -> Result<Vec<FileModel>> {
        let mut files: Vec<FileModel> = Vec::new();

        match self.walk_files() {
            Ok(mut file_list) => files.append(&mut file_list),
            Err(e) => error!("error walking files: {}", e),
        }
        match self.walk_folders() {
            Ok(mut file_list) => files.append(&mut file_list),
            Err(e) => error!("error walking files: {}", e),
        }

        Ok(files)
    }

    /// walk all the folders and files specified in config source folders and files
    pub fn walk_files(&self) -> Result<Vec<FileModel>> {
        info!("walk the folders and files");
        let mut files: Vec<FileModel> = Vec::new();

        for file in self.config.files.iter() {
            let pbuf: PathBuf = [&self.home, file].iter().collect();

            if pbuf.is_file() {
                let path = pbuf.as_path();
                if path.is_file() && path.exists() {
                    debug!("{}", &pbuf.display());
                    let model = FileModel::new(pbuf.to_str().unwrap());
                    let model = model.read_metadata()?;

                    files.push(model);
                }
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

                if path.is_symlink() {
                    debug!("symlink: {}", path.display());
                    continue;
                }

                if path.is_file() {
                    let modified = meta.modified()?;
                    let modified = modified.duration_since(std::time::SystemTime::UNIX_EPOCH)?;
                    debug!(
                        "{} {} {}",
                        &pbuf.display(),
                        meta.len(),
                        modified.as_micros()
                    );
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
