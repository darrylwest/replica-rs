use crate::config::Config;
use crate::file_model::FileModel;
use anyhow::Result;
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileWalker {
    config: Config,
    home: String,
}

impl FileWalker {
    /// create a new FileWalker
    pub fn new(config: Config) -> FileWalker {
        let home = config.clone().home;

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
            } else {
                error!("{} not found", pbuf.display());
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
                if self.exclude(entry.path()) || entry.file_name() == ".DS_Store" {
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
                    // debug!("{} {} {}", &pbuf.display(), meta.len(), modified.as_micros());
                    let model = FileModel::from(pbuf, meta.len(), modified.as_micros() as u64);
                    files.push(model);
                }
            }
        }

        Ok(files)
    }

    /// if the file path contains an exclude phrase return true, else false
    fn exclude(&self, path: &Path) -> bool {
        let excludes = &self.config.excludes;
        let name = path.to_str().unwrap().to_string();

        for ex in excludes.iter() {
            if name.contains(ex.as_str()) {
                debug!("exclude: {} {}", name, ex);
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excludes() {
        // [ ".config/broot", ".config/chromium", ".config/configstore", ".config/dconf" ]

        let config = Config::read_config(".test-replica/config/walk-config.toml").unwrap();
        let walker = FileWalker::new(config.clone());

        let path = Path::new("/home/dpw/.config/chromium/thing");
        assert!(walker.exclude(path));

        let path = Path::new(".config/configstore/");
        assert!(walker.exclude(path));
    }

    #[test]
    fn walk_files_and_folders() {
        // cd_test_home();
        let mut config = Config::read_config(".test-replica/config/walk-config.toml").unwrap();
        config.files = vec![String::from("bad.txt"), String::from("even-worse.txt")];
        config.source_folders = vec![String::from("bad.txt"), String::from("even-worse.txt")];
        let walker = FileWalker::new(config.clone());

        let files = walker.walk_files_and_folders().unwrap();
        println!("{:?}", files);
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn walk_folders() {
        // cd_test_home();
        let config = Config::read_config(".test-replica/config/walk-config.toml").unwrap();
        let walker = FileWalker::new(config.clone());

        let files = walker.walk_files_and_folders().unwrap();
        println!("{:?}", files);
        assert_eq!(files.len(), 5);
    }

    #[test]
    fn walk_files() {
        // cd_test_home();
        let config = Config::read_config(".test-replica/config/walk-config.toml").unwrap();
        println!("{:?}", config);
        let walker = FileWalker::new(config.clone());

        let file_count = walker.config.files.len();
        println!("{:?} count: {}", walker.config.files, file_count);

        let list = walker.walk_files().expect("should return ok");

        println!("{:?}", list);
        assert_eq!(list.len(), file_count);
    }
}
