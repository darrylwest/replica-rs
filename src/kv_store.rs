use crate::file_model::FileModel;
/// Key/Value Store - database operations
use anyhow::{anyhow, Result};
use hashbrown::HashMap;
use log::{error, info, warn};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
// use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub struct KvStore {
    dbpath: PathBuf,
    db: HashMap<String, FileModel>,
    index: HashMap<String, String>,
}

impl KvStore {
    /// initializes the database ; reads the dbfile, stores in k/v and creates index.
    pub fn init(dbpath: PathBuf) -> Result<KvStore> {
        let mut client = KvStore {
            dbpath,
            db: HashMap::new(),
            index: HashMap::new(),
        };

        match client.read_dbfile() {
            Ok(_) => Ok(client),
            Err(e) => {
                let msg = format!("error initializing database: {}", e);
                error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    fn read_dbfile(&mut self) -> Result<()> {
        info!("read database file from {}", self.dbpath.display());

        let file = match File::open(self.dbpath.as_os_str()) {
            Ok(file) => file,
            Err(e) => {
                warn!("New empty list: {}", e);
                return Ok(());
            }
        };

        let mut reader = BufReader::new(file);

        let mut text = String::new();
        reader.read_to_string(&mut text)?;

        let list: Vec<FileModel> = serde_json::from_str(&text)?;

        for model in list {
            let key = model.key.to_string();
            let mpath = model.path.to_str().unwrap();

            self.db.insert(key.clone(), model.clone());
            self.index.insert(mpath.to_string(), key);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let filename = "tests/data/files.json";
        let result = KvStore::init(PathBuf::from(filename));

        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn init_nofile() {
        let result = KvStore::init(PathBuf::from("tests/notarealfile.json"));

        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn init_bad_data() {
        let result = KvStore::init(PathBuf::from("tests/file1.txt"));

        println!("{:?}", result);
        assert!(result.is_err());
    }
}
