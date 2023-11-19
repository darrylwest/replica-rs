# Replica Backup

```bash
                   __ __                __                __                
.----.-----.-----.|  |__|.----.---.-.  |  |--.---.-.----.|  |--.--.--.-----.
|   _|  -__|  _  ||  |  ||  __|  _  |__|  _  |  _  |  __||    <|  |  |  _  |
|__| |_____|   __||__|__||____|___._|__|_____|___._|____||__|__|_____|   __|
           |__|                                                      |__|   
```

## Overview

Supervised file backups.

All machines run supervisor check all machines that have replica enabled to check for any errors.  Summary report(s) created and emailed.

## Data Flow Procedures

_TODO: Put this into a flow-charty thing_

* read from default or specified config file
* change to app home folder (usually HOME)
* walk the folders and files specified in config file
* iterate over the files comparing dates/sizes to backup dates/sizes
* write any files that need to be backed up
* run each 5 minutes

## Config

## Database

* json file(s) from serde/HashBrown
* read file system; do backups; write filelist
* file model struct with path, hash, len, modified and last_saved


## Issues

[Github Issue List](https://github.com/darrylwest/replica-rs/issues)
[Code Coverage Report](https://raincitysoftware.com/replica/)

###### dpw | 2023.11.19

