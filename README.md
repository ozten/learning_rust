# Learning Rust

A repo where I'm learning Rust and building a tool to analize github repositories.

## Setup
```
mkdir libs
cd libs
echo "Setting up http_client library"
git clone https://github.com/mozilla-servo/rust-http-client.git
cd rust-http-client
./configure
make
cd ..
echo "Setting up sqlite library"
git clone https://github.com/linuxfood/rustsqlite.git
cd rustsqlite
for src in pkg.rs types.rs ffi.rs cursor.rs database.rs sqlite.rc; do\
  rustc -L . --lib $src; done
cd ../../https_proxy
npm install
./proxy.js
```

## Build
```
rustc -L . -L libs/rustsqlite -L libs/rust-http-client main.rs
```

## Run

```
RUST_LOG=main=4 ./main
```

## TODO
* Setup a Task based system with main running two Tasks
  * github fetcher
  * sqlite db writer
* Put data into sqlite database
  * next repo link, raw sql
  * a row per repo

## Play
The directory `play` is useful in that it has various sample
code for different topics like Vectors, JSON, Tasks, etc.
All code works with Rust 0.8pre unless noted in the first comment.

### Database Schema

repository metadata (API cache)
id
url, next_url, date_retrieved, json
json - Jull original JSON

On startup - most recent repository metadata row
no metadata
url has metadata - retrieve next url
None - start retrieving

repositories
id, metadata_id, full_name, html_url
metadata_id - foreign key into repository_metadata
full_name - repository name account/repo


