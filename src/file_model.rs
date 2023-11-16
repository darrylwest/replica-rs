/// File Store - a small database of file names and hash values
///
use anyhow::Result;
use chrono::naive::NaiveDateTime;
use hashbrown::HashMap;
use log::{info, warn};
use openssl::sha;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileModel {
    pub path: PathBuf,
    pub hash: String,
    pub len: u64,
    pub modified: u64,
    pub last_saved: Option<NaiveDateTime>,
}

impl FileModel {
    pub fn new(path: &str) -> FileModel {
        FileModel {
            path: PathBuf::from(path),
            hash: "".into(),
            len: 0,
            modified: 0,
            last_saved: None,
        }
    }

    pub fn from(path: PathBuf, len: u64, modified: u64) -> FileModel {
        FileModel {
            path,
            hash: "".into(),
            len,
            modified,
            last_saved: None,
        }
    }

    /// calc the file's hash in hex format
    pub fn calc_hash(&self, content: &[u8]) -> String {
        let mut hasher = sha::Sha256::new();
        hasher.update(content);
        let hash = hasher.finish();

        hex::encode(hash)
    }

    /// read the db file
    pub fn read_dbfile(filename: &str) -> Result<HashMap<PathBuf, FileModel>> {
        // check to see if the file exists...
        info!("read dbfile: {}", filename);
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                warn!("creating a new empty hashmap: {}", e);
                let map: HashMap<PathBuf, FileModel> = HashMap::new();
                return Ok(map);
            }
        };

        let mut reader = BufReader::new(file);

        let mut text = String::new();
        reader.read_to_string(&mut text)?;

        let map: HashMap<PathBuf, FileModel> = serde_json::from_str(&text)?;

        Ok(map)
    }

    /// save the list of file models to disk
    pub fn write_dbfile(filename: &str, list: HashMap<PathBuf, FileModel>) -> Result<()> {
        info!("write models to file: {}", filename);
        let json = serde_json::to_string_pretty(&list).unwrap();

        let mut buf = File::create(filename)?;
        buf.write_all(json.as_bytes())?;

        Ok(())
    }

    /// strip off the home parts to return the relative path
    pub fn relative_path(&self) -> String {
        let mut home = env::var("HOME").expect("The user should have a home folder.");
        if !home.ends_with('/') {
            home.push('/');
        }
        self.path.to_str().unwrap().replace(home.as_str(), "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_hash() {
        let model = FileModel::new("config/config.toml");
        let content = std::fs::read("tests/big-file.pdf").unwrap();
        let hash = model.calc_hash(content.as_slice());

        println!("hash: {}", hash);
        assert_eq!(
            hash,
            "e23cd91ac0d728eec44d3c20b87accdb75ec7b9e67d35bad7fb8b672e0348d95"
        );
    }

    #[test]
    fn read_dbfile() {
        let filename = ".replica/data/files.json";
        let list = FileModel::read_dbfile(filename).expect("a vector of file models");

        assert_eq!(list.len(), 0);
    }
}
