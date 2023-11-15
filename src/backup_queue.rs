/// Backup Queue to save queued files to targets
///
/// create with target folder and queue vector; return the list of saved files updated with save date
///
use anyhow::Result;
// use chrono::{NaiveDate, NaiveDateTime};
use crate::file_model::FileModel;
use log::{info, warn};
use std::path::PathBuf;

pub struct BackupQueue {
    pub target: PathBuf,
    pub queue: Vec<FileModel>,
}

impl BackupQueue {
    pub fn new(path: &str, queue: Vec<FileModel>) -> BackupQueue {
        info!("create the backup queue.");
        BackupQueue {
            target: PathBuf::from(path),
            queue,
        }
    }

    pub fn process(&self) -> Result<Vec<FileModel>> {
        warn!("not implemented yet");
        let mut saved: Vec<FileModel> = Vec::new();

        for model in self.queue.clone() {
            // model.last_saved = Some(NaiveDateTime::
            saved.push(model);
        }

        Ok(saved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_queue() -> Vec<FileModel> {
        let queue: Vec<FileModel> = Vec::new();

        queue
    }

    #[test]
    fn new() {
        let path = "tests/";
        let queue = create_queue();

        let qlen = queue.len();
        let backup = BackupQueue::new(path, queue);
        assert_eq!(backup.queue.len(), qlen);
    }

    #[test]
    fn process() {
        let path = "tests/";
        let queue = create_queue();

        let qlen = queue.len();
        let backup = BackupQueue::new(path, queue);

        if let Ok(saved) = backup.process() {
            assert!(saved.len() == qlen);
        } else {
            assert!(false);
        }
    }
}
