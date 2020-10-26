# couchdb-design

A command line interface to work with CouchDB design documents as YAML configurations

### Usage 
```
USAGE:
    couchdb-design [FLAGS] [OPTIONS] <url>

FLAGS:
        --force      Force file creation
    -h, --help       Prints help information
    -q, --quiet      Supress diffs fore document views
    -t, --test       Just show diff, do not actually upload
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>  Local YAML file to be uploaded as design document. If not provided, it will read               URLs and display it as YAML file in stdout
    
```
### Installation

After cloning this repository, please run:
```
cargo install --path .
```

### License

MIT