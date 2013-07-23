# Learning Rust

A repo where I'm learning Rust and building a tool to analize github repositories.

## Setup
```
mkdir libs
cd libs
git clone https://github.com/mozilla-servo/rust-http-client.git
cd rust-http-client
./configure
make
```

## Build
```
rustc -L libs/rust-http-client hello.rs && ./hello
```

## Run

```
./hello
```

## TODO
* Setup a Task based system with main running two Tasks
  * github fetcher
  * sqlite db writer
* Put data into sqlite database
  * next repo link, raw sql
  * a row per repo


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


