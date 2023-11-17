use anyhow::{anyhow, Result};
use log::{debug, error, info};
use std::fs;
use std::path::{Path, PathBuf};
    pub files: Vec<FileModel>,
    pub fn new(path: &str, files: Vec<FileModel>, dryrun: bool) -> BackupQueue {
        let mut tp = path.to_string();
        if !tp.ends_with('/') {
            tp.push('/');
        }

        info!("dryrun = {}", dryrun);

            target: PathBuf::from(tp),
            files,
    /// process the file list; return the list of files that were backup
        info!("process the backup queue");
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
    fn create_filelist() -> Vec<FileModel> {
        let files: Vec<FileModel> = Vec::new();
        files
        let files = create_filelist();
        let flen = files.len();
        let backup = BackupQueue::new(path, files, true);
        assert_eq!(backup.files.len(), flen);
        let files = create_filelist();
        let flen = files.len();
        let backup = BackupQueue::new(path, files, true);
        assert_eq!(flen, backup.files.len());
        assert!(true);