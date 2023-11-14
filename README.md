# Replica Backup

```bash
                   __ __                __                __                
.----.-----.-----.|  |__|.----.---.-.  |  |--.---.-.----.|  |--.--.--.-----.
|   _|  -__|  _  ||  |  ||  __|  _  |__|  _  |  _  |  __||    <|  |  |  _  |
|__| |_____|   __||__|__||____|___._|__|_____|___._|____||__|__|_____|   __|
           |__|                                                      |__|   
```

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

###### dpw | 2023.11.14

