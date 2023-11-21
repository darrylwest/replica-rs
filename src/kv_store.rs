use crate::file_model::FileModel;
/// Key/Value Store - database operations
use anyhow::{anyhow, Result};
use hashbrown::HashMap;
use log::{error, info, warn};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
// use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub struct KeyValueStore {
    dbpath: PathBuf,
    db: HashMap<String, FileModel>,
    index: HashMap<String, String>,
    dirty_flag: bool,
}

impl KeyValueStore {
    /// initializes the database ; reads the dbfile, stores in k/v and creates index.
    pub fn init(dbpath: PathBuf) -> Result<KeyValueStore> {
        let mut client = KeyValueStore {
            dbpath,
            db: HashMap::new(),
            index: HashMap::new(),
            dirty_flag: false,
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

    /// get the file model or return None if it doesn't exist
    pub fn get(&self, key: &str) -> Option<&FileModel> {
        self.db.get(key)
    }

    /// insert the model into k/v store's db
    pub fn set(&mut self, model: FileModel) -> Result<()> {
        self.dirty_flag = true;
        let key = model.key.to_string();
        let _ = self.db.insert(key, model);

        Ok(())
    }

    /// return the size of this database
    pub fn dbsize(&self) -> usize {
        self.db.len()
    }

    /// return true if the data has been updated, else false
    pub fn is_dirty(&self) -> bool {
        self.dirty_flag
    }

    /// find the file model from the path
    pub fn find(&self, path: &str) -> Option<&FileModel> {
        let key = self.index.get(path);
        key?;

        self.db.get(key.unwrap())
    }

    /// save the kv to file
    pub fn savedb(&mut self, filename: &str) -> Result<()> {
        info!("save the k/v models as a list to file: {}", filename);
        let list: Vec<FileModel> = self.db.clone().into_values().collect();
        let json = serde_json::to_string_pretty(&list).unwrap();

        match File::create(filename) {
            Ok(mut buf) => buf.write_all(json.as_bytes())?,
            Err(e) => {
                let msg = format!("dbfile write error: {}, {}", filename, e);
                error!("{}", msg);
                return Err(anyhow!("{}", msg));
            }
        }

        info!("reset the dirty flag to false");
        self.dirty_flag = false;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn savedb_bad() {
        let filename = "tests/data/files.json";
        let mut client = KeyValueStore::init(PathBuf::from(filename)).unwrap();

        let filename = "tests/no-data/bad/backup.json";
        let result = client.savedb(filename);
        assert!(result.is_err())
    }

    #[test]
    fn savedb() {
        let filename = "tests/data/files.json";
        let mut client = KeyValueStore::init(PathBuf::from(filename)).unwrap();

        let filename = "tests/data/backup.json";
        let result = client.savedb(filename);
        assert!(result.is_ok())
    }

    #[test]
    fn find() {
        let filename = "tests/data/files.json";
        let client = KeyValueStore::init(PathBuf::from(filename)).unwrap();

        let model = client.find("a/bad/path");
        assert!(model.is_none());

        let model = client.find("./tests/big-file.pdf");
        assert!(model.is_some());
        let model = model.unwrap();
        assert_eq!(model.key, "Npeu7mr2B2ua25Sn");
    }

    #[test]
    fn getset_dirty_dbsize() {
        let filename = "tests/data/files.json";
        let mut client = KeyValueStore::init(PathBuf::from(filename)).unwrap();
        assert!(!client.is_dirty());
        let count: usize = 5;
        assert_eq!(client.dbsize(), count);

        let result = client.get("bad-key");
        assert!(result.is_none());

        let result = client.get("8iwl7mr2DU3XnMkT");
        println!("{:?}", result);
        assert!(result.is_some());
        let model = result.unwrap();
        assert_eq!(model.len, 19);
        assert!(!client.is_dirty());

        let mut model = model.clone();
        let myhash = "1234567";
        model.hash = myhash.to_string();
        assert_eq!(&model.hash, &myhash);

        let result = client.set(model.clone());
        assert!(result.is_ok());
        assert!(client.is_dirty());

        let updated = client.get(&model.key).unwrap();
        println!("up: {:?}", updated);
        assert_eq!(updated.hash, myhash);
        assert_eq!(client.dbsize(), count);
    }

    #[test]
    fn init() {
        let filename = "tests/data/files.json";
        let result = KeyValueStore::init(PathBuf::from(filename));

        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn init_nofile() {
        let result = KeyValueStore::init(PathBuf::from("tests/notarealfile.json"));

        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn init_bad_data() {
        let result = KeyValueStore::init(PathBuf::from("tests/file1.txt"));

        println!("{:?}", result);
        assert!(result.is_err());
    }
}
