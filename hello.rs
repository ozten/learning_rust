extern mod extra;
extern mod http_client;



use extra::json;
use extra::net::url::Url;
use extra::net::url;
use std::hashmap::HashMap;
use std::str;
use std::result::{Result, Ok, Err};

use http_client::uv_http_request;
use http_client::RequestEvent;


fn main() {

    // http://ozten.com/psto/
    // https://api.github.com/repositories
    let u: Url = url::from_str("https://api.github.com/repositories").get();
    let mut request = uv_http_request(u);

    // Problem #1 a closure with state to build on the response
    // into a string
    fn mk_handleEvent(rawJson:~str) -> @fn(RequestEvent) -> () {
        return |event| {

            //rawJson = rawJson + ~"huh";

            match event {
                http_client::Error(e) => {
                    println(fmt!("Ouch... error %?", e));
                },
                http_client::Status(s) => {
                    println(fmt!("Status %?", s));
                    //let res = json::from_str(rawJson);
                    println(fmt!("Payload! %?", rawJson));
                },
                http_client::Payload(p) => {
                    let data = p.take();
                    //rawJson +=  str::from_bytes(data);

                }
            }
        };
    };

    do request.begin |event| {

    }

    // Problem #2 How to output parts of JSON after parsing
    // I think this is working...
    match json::from_str("{\"ham\": \"bone\"}") {
        Ok(obj) => {
            match obj {
                json::Object(o) => {
                    println(fmt!("Payload! %?", o.get(& ~"ham")))
                }
                _ => { println("Skipping") }
            }

        }
        Err(e) => {
            println("Error")
        }
    }

    // HashMap play - One type of Json is Object which is a
    // type over HashMap<~str, Json>
    let mut h: HashMap<~str, ~str> = HashMap::new();
    h.insert(~"foo", ~"bar");
    h.get(& ~"foo");
}