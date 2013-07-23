// rustc -L ../libs/rust-http-client http.rs && RUST_LOG=http=4 ./http

extern mod extra;
extern mod http_client;

use std::hashmap::HashMap;
use std::str;

use extra::net::url;
use extra::net::url::Url;

use http_client::uv_http_request;
use http_client::StatusCode;

fn main() {
    // http://developer.github.com/v3/
    // https://api.github.com/repositories
    // http://ozten.com/random/sample.json

    let u: Url = url::from_str("http://localhost:8002/repositories").get();
    let mut options:HashMap<~str, ~str> = HashMap::new();
    options.insert(~"User-Agent",
                   ~"curl/7.21.4 (universal-apple-darwin11.0) libcurl/7.21.4 OpenSSL/0.9.8x zlib/1.2.5");
    options.insert(~"Accept", ~"*/*");
    let mut request = uv_http_request(u, options);
    do request.begin |event| {
        match event {
            http_client::Error(e) => {
                println(fmt!("Ouch... error %?", e));
            },
            http_client::Status(s) => {
                match s {
                    // TODO wait... how did I break how match works here
                    // I should need the pattern guard.
                    StatusOK if s == StatusOK => {
                        println(fmt!("Status %?", s));
                    }
                    _ => {}
                }
            },
            http_client::Payload(p) => {
                let data = p.take();
                //println(str::from_bytes(data));
            }
        }
    }
}