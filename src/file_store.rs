use crate::config::Config;
use chrono::naive::NaiveDateTime;
use hashbrown::HashMap;
/// File Store - a small database of file names and hash values
///
use log::info;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct FileModel {
    path: String,
    current_hash: String,
    old_hash: String,
    last_saved_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum Command {
    Get(String, oneshot::Sender<Option<FileModel>>),
    Put(FileModel, oneshot::Sender<String>),
    Delete(String, oneshot::Sender<String>),
    List(oneshot::Sender<Vec<FileModel>>),
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
                        let _ = tx.send("ok".to_string());
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
                        let mut list = Vec::with_capacity(map.len());
                        for (_, value) in map.iter() {
                            list.push(value.clone());
                        }

                        let _ = tx.send(list);
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
}
