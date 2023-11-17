# Replica Backup

```bash
                   __ __                __                __                
.----.-----.-----.|  |__|.----.---.-.  |  |--.---.-.----.|  |--.--.--.-----.
|   _|  -__|  _  ||  |  ||  __|  _  |__|  _  |  _  |  __||    <|  |  |  _  |
|__| |_____|   __||__|__||____|___._|__|_____|___._|____||__|__|_____|   __|
           |__|                                                      |__|   
```

## Overview

Supervised backups...

All machines run supervisor check all machines that have replica enabled to check for any errors.  Summary report(s) created and emailed.

## To Do

* [x] add process.rs and mv main.rs to bin/replica.rs
* [x] add CLI
* [x] run from crontab to test
* [x] modify tests to remove $HOME dependency
* [x] change logging to only log what was queued and backed up
* [x] implement queue vector and backup_queue module to 
* [x] save files to backup target
* [ ] modify to write to all targets (if possible)
* [x] improve error handling
* [ ] replace string manupulation for std::path::{Path, PathBuf}
* [x] implement file writes to target backup
* [ ] add exclude or ignore patterns in config
* [ ] implement supervisor to check backups by reading data/files.json and generate a status report (daily)

## Data Flow Procedures

* change to home folder
* from config, read the file list (if any)
* read the db and update with any changes
* queue changed files
* drain queue by saving new files while updating database
* rotate db then write current
* run each 5 minutes?

## Database

* json file from serde/HashBrown
* read all; make updates; write/store all
* file model struct with path, hash, len, modified and last_saved

###### dpw | 2023.11.16

