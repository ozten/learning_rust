extern mod extra;
extern mod http_client;

use std::hashmap::HashMap;
use std::str;

use extra::net::url;
use extra::net::url::Url;

use http_client::uv_http_request;
use http_client::StatusCode;

fn main() {
    let u: Url = url::from_str("http://ozten.com/").get();
    let mut options:HashMap<~str, ~str> = HashMap::new();
    options.insert(~"User-Agent", ~"SpamBot 5000");
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