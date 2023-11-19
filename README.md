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
* run each 2 to 5 minutes

## Config

## Database

A small, static database is created with each backup run.  It can be used to see what files are in the backup list with stats on size, modified date, and last saved.

* creates vector of [FileModel](file:///Users/dpw/raincity/rust-projects/replica/target/doc/replica/file_model/index.html) entries
* writes the vector in json format to ./data folder

## Roadmap

This project is in it's early stage.  There are plenty of [issues](https://github.com/darrylwest/replica-rs/issues) that need to 
be resolved prior to going to an actual production-ish stage. And by _production-ish_ I mean it is targeted to linux and osx, so
windows (ms-dos, whatever) probably isn't on the map, unless someone with a windows machine contributes.

When the project is stable enough to go live, it's version will reach version 1.0.0  Until then, anything can--and probably will change.

## Issues

[Github Issue List](https://github.com/darrylwest/replica-rs/issues) |
[Code Coverage Report](https://raincitysoftware.com/replica/)

###### dpw | 2023.11.19

