# Replica Backup

```bash
                   __ __                __                __                
.----.-----.-----.|  |__|.----.---.-.  |  |--.---.-.----.|  |--.--.--.-----.
|   _|  -__|  _  ||  |  ||  __|  _  |__|  _  |  _  |  __||    <|  |  |  _  |
|__| |_____|   __||__|__||____|___._|__|_____|___._|____||__|__|_____|   __|
           |__|                                                      |__|   
```

## Data Flow Procedures

* from config, read the file list and create/update the database
* iterate over all the files to determine if any have changed
* queue changed files
* drain queue by saving new files while updating database

## Database

* json file
* sqlite3 (no service required)
* redis
* custom kv store
* other static file k/v

###### dpw | 2023.11.11

