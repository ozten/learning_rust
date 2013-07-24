
// rustc -L . -L libs/rust-http-client -L libs/rustsqlite/ main.rs && RUST_LOG=main=4 ./main

extern mod db;
extern mod repository_metadata_slurp;

use db::*;
use repository_metadata_slurp::slurp_repos;

fn main() {
    let (start_port, start_chan): (Port<~str>, Chan<~str>) = stream();
    do spawn {
        init_db(&start_chan);
    }

    do spawn {
        // Wait for starting url from db
        slurp_repos(start_port.recv());
    }
}