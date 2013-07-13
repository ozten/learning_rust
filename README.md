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