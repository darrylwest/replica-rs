/// File Model - a small database of file names and hash values
///
/// # file Model
///
use anyhow::{anyhow, Result};
use chrono::naive::NaiveDateTime;
use domain_keys::keys::RouteKey;
use hashbrown::HashSet;
use log::error;
use openssl::sha;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileModel {
    pub key: String,
    pub path: PathBuf,
    pub hash: String,
    pub len: u64,
    pub modified: u64,
    pub last_saved: Option<NaiveDateTime>,
    pub written_to: HashSet<String>,
}

impl FileModel {
    pub fn new(path: &str) -> FileModel {
        FileModel {
            key: RouteKey::create(),
            path: PathBuf::from(path),
            hash: "".into(),
            len: 0,
            modified: 0,
            last_saved: None,
            written_to: HashSet::new(),
        }
    }

    pub fn from(path: PathBuf, len: u64, modified: u64) -> FileModel {
        FileModel {
            key: RouteKey::create(),
            path,
            hash: "".into(),
            len,
            modified,
            last_saved: None,
            written_to: HashSet::new(),
        }
    }

    /// copy constructor
    pub fn copy_from(model: FileModel) -> FileModel {
        FileModel {
            key: model.key.clone(),
            path: model.path.clone(),
            hash: model.hash.clone(),
            len: model.len,
            modified: model.modified,
            last_saved: model.last_saved,
            written_to: model.written_to,
        }
    }

    /// read the file based metadata, len, modified, etc
    pub fn read_metadata(&self) -> Result<FileModel> {
        let mut model = self.clone();
        let resp = self.path.metadata();
        if resp.is_err() {
            let msg = format!("failed to get meta data: {:?}", resp);
            error!("{}", msg);
            return Err(anyhow!("{}", msg));
        }

        let meta = resp.unwrap();
        model.len = meta.len();

        let modified = meta.modified().unwrap();
        model.modified = modified
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        Ok(model)
    }

    /// calc the file's hash in hex format
    pub fn calc_hash(&self, content: &[u8]) -> String {
        let mut hasher = sha::Sha256::new();
        hasher.update(content);
        let hash = hasher.finish();

        hex::encode(hash)
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
}
