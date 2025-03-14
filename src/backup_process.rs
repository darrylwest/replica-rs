/// Backup Queue to save queued files to targets
///
/// # Backup Process
///
/// create with target folder and queue vector; return the list of saved files updated with save date
///
use crate::file_model::FileModel;
use crate::kv_store::KeyValueStore;
use anyhow::{anyhow, Result};
use chrono::Utc;
use log::{debug, error, info, warn};
use std::fs;
use std::path::{Path, PathBuf};

pub struct BackupProcess {
    pub target: PathBuf,
    pub files: Vec<FileModel>,
    pub dryrun: bool,
}

impl BackupProcess {
    pub fn new(path: &str, files: Vec<FileModel>, dryrun: bool) -> BackupProcess {
        let mut tp = path.to_string();
        if !tp.ends_with('/') {
            tp.push('/');
        }

        info!("dryrun = {}", dryrun);

        BackupProcess {
            target: PathBuf::from(tp),
            files,
            dryrun,
        }
    }

    /// return true if the target exists, else false
    pub fn target_exists(&self) -> bool {
        if self.target.exists() && self.target.is_dir() {
            true
        } else {
            warn!("Target {:?} does not exist.", self.target);
            false
        }
    }

    /// process the file list; return the list of files that were backup
    pub fn process(&self, mut db: KeyValueStore) -> Result<KeyValueStore> {
        info!("process the backup queue");

        let files = self.files.clone();
        for file_model in files {
            let fpath = file_model.path.as_os_str();
            match self.check_and_copy_file(&file_model) {
                Some(saved_model) => {
                    info!("file backup: {:?} -> {}", fpath, saved_model.path.display());

                    // save to db
                    let resp = db.set(saved_model.clone());
                    if resp.is_err() {
                        error!("could not save to database: {:?}", resp);
                    } else {
                        info!("saved to db: {:?}", saved_model);
                    }
                }
                None => debug!("skip {:?}", fpath),
            }
        }

        Ok(db)
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

        match self.copy_model(model, target_model) {
            Ok(model) => Some(model),
            Err(_e) => None,
        }
    }

    /// return a new file model if the two don't match or the target does not exist
    pub fn match_files(&self, ref_model: &FileModel, target_path: &Path) -> Option<FileModel> {
        let filename = target_path.to_str().unwrap();
        let mut target_model = FileModel::new(filename);
        // this ensures that the last_updated gets set to the correct record
        target_model.key = ref_model.key.clone();

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

    /// Copy the source to destination; update the source last_saved date and written to hash;
    /// Return the updated src model
    pub fn copy_model(&self, src: &FileModel, dest: FileModel) -> Result<FileModel> {
        let save_model = FileModel::copy_from(dest);

        if self.dryrun {
            return Ok(save_model);
        }

        let src_path = src.path.as_path();
        let dest_path = save_model.path.as_path();
        if self.copy(src_path, dest_path).is_err() {
            let msg = format!("error saving to: {}", dest_path.display());
            error!("{}", msg);
            Err(anyhow!("{}", msg))
        } else {
            let now = Utc::now().naive_utc();
            let write_path = dest_path.to_str().unwrap();

            let mut model = src.clone();

            model.last_saved = Some(now);
            model.written_to.insert(write_path.to_string());

            Ok(model)
        }
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

    /// retrun the UTC time in seconds (unix timestamp.) decode with date -r <seconds>
    pub fn timestamp(&self) -> u64 {
        Utc::now().timestamp() as u64
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
        let backup = BackupProcess::new(path, files, true);
        assert_eq!(backup.files.len(), flen);
    }

    #[test]
    fn process() {
        let path = "tests/";
        let files = create_filelist();

        let flen = files.len();
        let backup = BackupProcess::new(path, files, true);
        assert_eq!(flen, backup.files.len());

        assert!(true);
    }

    #[test]
    fn copy_model() {
        let src = FileModel::new("tests/file3.txt");
        let dest = FileModel::new("tests/tback/file3.txt");
        println!("src: {:?}, dest: {:?}", src, dest);

        let backup = BackupProcess::new("./", vec![], false);
        let response = backup.copy_model(&src, dest);

        println!("{:?}", response);
        assert!(response.is_ok());
    }

    #[test]
    fn bad_copy_model() {
        let src = FileModel::new("tests/file-nofile.txt");
        let dest = FileModel::new("tests/tback/file-nofile.txt");
        println!("src: {:?}, dest: {:?}", src, dest);

        let backup = BackupProcess::new("./", vec![], false);
        let response = backup.copy_model(&src, dest);

        println!("{:?}", response);
        assert!(response.is_err());
    }

    #[test]
    fn copy_new_folder() {
        let src = Path::new("tests/file3.txt");
        let dest = Path::new("tests/tback-tmp/file3.txt");

        println!("src: {}, dest: {:?}", src.display(), dest.display());

        let backup = BackupProcess::new("./", vec![], false);
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

        let backup = BackupProcess::new("./", vec![], true);
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

        let backup = BackupProcess::new("./", vec![], true);
        let response = backup.match_files(&src, dest);

        println!("{:?}", response);
        assert!(response.is_some());
    }

    #[test]
    fn timestamp() {
        let path = "tests/";
        let files = create_filelist();

        let backup = BackupProcess::new(path, files, true);

        let ts = backup.timestamp();

        println!("timestamp: {} = {}", ts, get_now_seconds());
        assert!(ts > 1_700_604_600);
        let tss = from_timestamp(ts);
        println!("{}->{}", ts, tss);
    }

    fn get_now_seconds() -> u64 {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn from_timestamp(ts: u64) -> String {
        use chrono::TimeZone;
        Utc.timestamp_opt(ts as i64, 0).unwrap().to_rfc3339()
    }
}
