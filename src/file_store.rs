/// File Store - a small database of file names and hash values
///
use crate::config::Config;
use chrono::naive::NaiveDateTime;
use hashbrown::HashMap;
use log::info;
use openssl::sha;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct FileModel {
    path: String,
    hash: String,
    len: u64,
    modified: u64,
    last_saved: Option<NaiveDateTime>,
}

impl FileModel {
    pub fn new(path: &str) -> FileModel {
        FileModel {
            path: path.into(),
            hash: "".into(),
            len: 0,
            modified: 0,
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
}

#[derive(Debug)]
pub enum Command {
    Get(String, oneshot::Sender<Option<FileModel>>),
    Put(FileModel, oneshot::Sender<String>),
    Delete(String, oneshot::Sender<String>),
    List(oneshot::Sender<Vec<FileModel>>),
    SaveDb(oneshot::Sender<String>),
}

#[derive(Debug)]
pub struct FileStore {
    req_sender: mpsc::Sender<Command>,
}

impl FileStore {
    pub async fn new(config: &Config) -> FileStore {
        info!("start the file store/db: config: {:?}", &config);

        let req_sender: mpsc::Sender<Command>;
        let mut req_receiver: mpsc::Receiver<Command>;

        (req_sender, req_receiver) = mpsc::channel(64);

        let mut map = FileStore::load_file_db("data/filedb.json");
        // read the current filelist db...

        tokio::spawn(async move {
            while let Some(cmd) = req_receiver.recv().await {
                info!("req recv: {:?}", &cmd);
                match cmd {
                    Command::Put(model, tx) => {
                        info!("put file model: {:?}", &model);
                        let file_model = model.clone();
                        map.insert(model.path, file_model);
                        let _ = tx.send("ok".into());
                    }
                    Command::Get(path, tx) => {
                        info!("get file model: {}", &path);
                        let _ = if let Some(v) = map.get(&path) {
                            tx.send(Some(v.clone()))
                        } else {
                            tx.send(None)
                        };
                    }
                    Command::Delete(path, tx) => {
                        map.remove(&path);
                        let _ = tx.send("ok".into());
                    }
                    Command::List(tx) => {
                        let list: Vec<FileModel> = map.values().cloned().collect();
                        let _ = tx.send(list);
                    }
                    Command::SaveDb(tx) => {
                        let json = serde_json::to_string(&map).expect("a json map");
                        info!("json: {}", json);

                        let _ = tx.send("ok".into());
                    }
                }

                info!("db: {:?}", map);
            }

            req_receiver.close();
        });

        FileStore { req_sender }
    }

    pub fn request_channel(&self) -> mpsc::Sender<Command> {
        self.req_sender.clone()
    }

    /// load the file models db from json file
    pub fn load_file_db(_filename: &str) -> HashMap<String, FileModel> {
        // load the json file and insert into map
        let map: HashMap<String, FileModel> = HashMap::new();

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file_db() {
        let db = FileStore::load_file_db("tests/data/filedb.json");

        assert_eq!(db.len(), 0);
    }

    #[test]
    fn save_db() {
        let map = create_db();

        let json = serde_json::to_string(&map).unwrap();
        println!("map: {}", json);
    }

    #[test]
    fn list() {
        let map = create_db();

        let list: Vec<FileModel> = map.values().cloned().collect();
        println!("vec: {:?}", list);

        assert_eq!(list.len(), map.len())
    }

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

    fn create_db() -> HashMap<String, FileModel> {
        let mut map: HashMap<String, FileModel> = HashMap::new();

        for idx in 1..=5 {
            let model = FileModel::new(&format!("file-{}", idx));
            map.insert(model.path.to_string(), model.clone());
        }

        map
    }
}
