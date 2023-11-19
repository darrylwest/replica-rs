/// Backup Queue to save queued files to targets
///
/// # Backup Queue
///
/// create with target folder and queue vector; return the list of saved files updated with save date
///
use crate::file_model::FileModel;
use anyhow::{anyhow, Result};
use chrono::Utc;
use log::{debug, error, info};
use std::fs;
use std::path::{Path, PathBuf};

pub struct BackupQueue {
    pub target: PathBuf,
    pub files: Vec<FileModel>,
    pub dryrun: bool,
}

impl BackupQueue {
    pub fn new(path: &str, files: Vec<FileModel>, dryrun: bool) -> BackupQueue {
        let mut tp = path.to_string();
        if !tp.ends_with('/') {
            tp.push('/');
        }

        info!("dryrun = {}", dryrun);

        BackupQueue {
            target: PathBuf::from(tp),
            files,
            dryrun,
        }
    }

    /// process the file list; return the list of files that were backup
    pub fn process(&self) -> Result<Vec<FileModel>> {
        info!("process the backup queue");
        let mut saved: Vec<FileModel> = Vec::new();

        let files = self.files.clone();
        for file_model in files {
            let fpath = file_model.path.as_os_str();
            match self.check_and_copy_file(&file_model) {
                Some(backup_model) => {
                    info!("backup: {:?} -> {}", fpath, backup_model.path.display());

                    saved.push(backup_model);
                }
                None => debug!("skip {:?}", fpath),
            }
        }

        Ok(saved)
    }

    /// create the target path; check stat to see backup is required
    pub fn check_and_copy_file(&self, model: &FileModel) -> Option<FileModel> {
        let relative_path = model.relative_path();
        let target_path = Path::join(self.target.as_path(), PathBuf::from(relative_path));

        debug!("target path: {}", target_path.to_string_lossy());

        // if the file exists, check the size and modfied dates; if different then
        let target_model = self.match_files(model, target_path.as_path());
        target_model.as_ref()?;

        let target_model = target_model.unwrap();

        match self.copy_model(model.path.as_path(), target_model) {
            Ok(model) => Some(model),
            Err(_e) => None,
        }
    }

    /// return a new file model if the two don't match or the target does not exist
    pub fn match_files(&self, ref_model: &FileModel, target_path: &Path) -> Option<FileModel> {
        let filename = target_path.to_str().unwrap();
        let mut target_model = FileModel::new(filename);

        if target_path.exists() {
            target_model = target_model.read_metadata().unwrap();
            // assume they are the same...

            // TODO: need to compare hashes
            if target_model.len == ref_model.len {
                // && target_model.modified < ref_model.modified {
                return None;
            }
        }

        Some(target_model)
    }

    /// copy the source to destination and return the updated model
    pub fn copy_model(&self, src: &Path, dest: FileModel) -> Result<FileModel> {
        let mut save_model = FileModel::copy_from(dest);

        if self.dryrun {
            return Ok(save_model);
        }

        let dest_path = save_model.path.as_path();
        if self.copy(src, dest_path).is_err() {
            let msg = format!("error saving to: {}", dest_path.display());
            error!("{}", msg);
            return Err(anyhow!("{}", msg));
        }

        let now = Utc::now().naive_utc();
        save_model.last_saved = Some(now);

        Ok(save_model)
    }

    /// copy from src to dest
    pub fn copy(&self, src: &Path, dest: &Path) -> Result<()> {
        let parent = dest.parent().expect("the destination shoul have a parent");

        if !parent.exists() {
            info!("create the parent folder: {:?}", &parent);
            if fs::create_dir_all(parent).is_err() {
                let msg = format!("error creating parent folder: {}", parent.display());
                error!("{}", msg);
                return Err(anyhow!("{}", msg));
            }
        }

        if fs::copy(src, dest).is_err() {
            let msg = format!("error copying {} to {}", src.display(), dest.display());
            error!("{}", msg);
            return Err(anyhow!("{}", msg));
        }

        Ok(())
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

    #[test]
    fn copy_model() {
        let src = Path::new("tests/file3.txt");
        let dest = FileModel::new("tests/tback/file3.txt");
        println!("src: {}, dest: {:?}", src.display(), dest);

        let backup = BackupQueue::new("./", vec![], false);
        let response = backup.copy_model(src, dest);

        println!("{:?}", response);
        assert!(response.is_ok());
    }

    #[test]
    fn copy_new_folder() {
        let src = Path::new("tests/file3.txt");
        let dest = Path::new("tests/tback-tmp/file3.txt");

        println!("src: {}, dest: {:?}", src.display(), dest.display());

        let backup = BackupQueue::new("./", vec![], false);
        let response = backup.copy(src, dest);

        println!("{:?}", response);
        assert!(response.is_ok());
    }

    #[test]
    fn match_files() {
        let src = FileModel::new("tests/file2.txt");
        let src = src.read_metadata().unwrap();
        let dest = Path::new("tests/tback/file2.txt");
        println!("src: {:?}, dest: {}", src, dest.display());

        let backup = BackupQueue::new("./", vec![], true);
        let response = backup.match_files(&src, dest);

        println!("{:?}", response);
        assert!(response.is_none());
    }

    #[test]
    fn match_different_files() {
        let src = FileModel::new("tests/file1.txt");
        let src = src.read_metadata().unwrap();
        let dest = Path::new("tests/tback/file1.txt");
        println!("src: {:?}, dest: {}", src, dest.display());

        let backup = BackupQueue::new("./", vec![], true);
        let response = backup.match_files(&src, dest);

        println!("{:?}", response);
        assert!(response.is_some());
    }
}
