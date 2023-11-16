use crate::file_model::FileModel;
/// Backup Queue to save queued files to targets
///
/// create with target folder and queue vector; return the list of saved files updated with save date
///
use anyhow::Result;
// use chrono::Utc;
use log::{debug, info, warn};
use std::path::{Path, PathBuf};

pub struct BackupQueue {
    pub target: PathBuf,
    pub files: Vec<FileModel>,
    pub dryrun: bool,
}

impl BackupQueue {
    pub fn new(path: &str, files: Vec<FileModel>, dryrun: bool) -> BackupQueue {
        info!("create the backup queue.");
        let mut tp = path.to_string();
        if !tp.ends_with('/') {
            tp.push('/');
        }

        BackupQueue {
            target: PathBuf::from(tp),
            files,
            dryrun,
        }
    }

    /// process the file list; return the list of files that were backup
    pub fn process(&self) -> Result<Vec<FileModel>> {
        warn!("not implemented yet");
        let mut saved: Vec<FileModel> = Vec::new();
        let files = self.files.clone();
        for file_model in files {
            let fpath = file_model.path.as_os_str();
            match self.check_file(&file_model) {
                Some(backup_model) => {
                    info!("backup: {:?}", backup_model);
                    saved.push(backup_model);
                }
                None => debug!("skip {:?}", fpath),
            }

            // saved.push(model);
        }

        // let now = Utc::now().naive_utc();
        // model.last_saved = Some(now);

        Ok(saved)
    }

    /// create the target path; check stat to see backup is required
    pub fn check_file(&self, model: &FileModel) -> Option<FileModel> {
        let relative_path = model.relative_path();
        let target_path = Path::join(self.target.as_path(), PathBuf::from(relative_path));
        info!("target path: {}", target_path.to_string_lossy());

        // read the target

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_filelist() -> Vec<FileModel> {
        let files: Vec<FileModel> = Vec::new();

        files
    }

    #[test]
    fn new() {
        let path = "tests/";
        let files = create_filelist();

        let flen = files.len();
        let backup = BackupQueue::new(path, files, true);
        assert_eq!(backup.files.len(), flen);
    }

    #[test]
    fn process() {
        let path = "tests/";
        let files = create_filelist();

        let flen = files.len();
        let backup = BackupQueue::new(path, files, true);
        assert_eq!(flen, backup.files.len());

        assert!(true);
    }
}
