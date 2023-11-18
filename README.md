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

## Issues

### In Process

* [-] improve error handling
* [-] improve unit tests

### Planned

* [ ] find a substitute for crontab on mac to get around prompt to write to external devices on mac
* [ ] modify to write to all targets in the list (if possible)
* [ ] replace string manupulation for std::path::{Path, PathBuf}
* [ ] add exclude or ignore patterns in config
* [ ] add unit tests to get > 95% coverage
* [ ] determine how to do journaled backups to enable going back to an earlier version
* [ ] implement replica-monitor to check backups by reading data/files.json; generate a web report
* [ ] implement new-files.rs to list files that are not in being backed up

### Completed

* [x] add process.rs and mv main.rs to bin/replica.rs
* [x] add CLI
* [x] run from crontab to test
* [x] modify tests to remove $HOME dependency
* [x] change logging to only log what was queued and backed up
* [x] implement queue vector and backup_queue module to 
* [x] save files to backup target
* [x] log and error when a specified file is not found
* [x] implement file writes to target backup
* [x] add version to begining log message

## Data Flow Procedures

* read from default or specified config file
* change to app home folder (usually HOME)
* walk the folders and files specified in config file
* iterate over the files comparing dates/sizes to backup dates/sizes
* write any files that need to be backed up
* run each 5 minutes

## Database

* json file(s) from serde/HashBrown
* read file system; do backups; write filelist
* file model struct with path, hash, len, modified and last_saved

###### dpw | 2023.11.17

