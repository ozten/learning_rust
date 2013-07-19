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
* Send User-Agent in http header
* Examine http headers for next url in retrieving rep JSON
* Put data into sqlite database


### Database Schema
repositories
id, full_name, html_url, metadata
full_name - repository name account/repo
metadata - Jull original JSON