
// rustc -L . -L libs/rust-http-client -L libs/rustsqlite/ main.rs && RUST_LOG=main=4 ./main
extern mod extra;

extern mod db;
extern mod repository_metadata_slurp;

use extra::comm::DuplexStream;

use db::*;
use repository_metadata_slurp::slurp_repos;

fn main() {
    let (db_child, repo_child):(DuplexStream<~str, ~str>, DuplexStream<~str, ~str>) = DuplexStream();
    do spawn {
        init_db(&db_child);
    }

    do spawn {
        // Wait for starting url from db
        slurp_repos(&repo_child);
    }
}