/// DbOps - database operations

use anyhow::{anyhow, Result};
use hashbrown::HashMap;
use std::path::PathBuf;
use crate::file_model::FileModel;
use log::{info, warn, error};
use std::fs::File;
use std::io::{BufReader, Read};
// use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub struct DbOps {
    dbpath: PathBuf,
    db: HashMap<String, FileModel>,
    index: HashMap<String, String>,
}

impl DbOps {
    /// initializes the database ; reads the dbfile, stores in k/v and creates index.
    pub fn init(dbpath: PathBuf) -> Result<DbOps> {
        let mut client = DbOps {
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
        let result = DbOps::init(PathBuf::from(filename));

        println!("{:?}", result);
    }
}