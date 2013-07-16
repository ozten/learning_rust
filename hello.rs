extern mod extra;
extern mod http_client;



use extra::json;
use extra::net::url::Url;
use extra::net::url;
use std::hashmap::HashMap;
use std::str;
use std::result::{Result, Ok, Err};
use std::vec;

use http_client::uv_http_request;
use http_client::RequestEvent;

pub struct RepoResponse {
    rawJson: ~[~str]
}

impl RepoResponse {
    fn add(&mut self, data: ~str) {
        self.rawJson.push(data)
    }
}

fn main() {

    // http://ozten.com/psto/
    // https://api.github.com/repositories
    let u: Url = url::from_str("http://ozten.com/random/sample.json").get();
    let mut request = uv_http_request(u);

    let res = @mut RepoResponse{rawJson: ~[]};

    do request.begin |event| {
        match event {
            http_client::Error(e) => {
                println(fmt!("Ouch... error %?", e));
            },
            http_client::Status(s) => {
                //println(fmt!("Status %?", s));

                //res.rawJson.push(fmt!(" status event [%?] ", s));
                println(fmt!("Final payload was \n=========\n%?\n===========", res.rawJson.concat()));

                match json::from_str(res.rawJson.concat()) {
                    Ok(json) => {
                        match json {
                            json::Object(o) => {
                                println("Have at it boys")
                            }
                            json::List(l) => {
                                println("A list of Objects, perhaps")
                            }
                            _ => {
                                println("ERROR: Unrecognized JSON format")
                            }
                        }
                    }
                    Err(e) => {

                        println(fmt!("Error parsing JSON %?", e))
                    }
                }
            },
            http_client::Payload(p) => {
                let data = p.take();
                res.rawJson.push(str::from_bytes(data));
            }
        }
    }

    // Problem #2 How to output parts of JSON after parsing
    // I think this is working...
    match json::from_str("{\"ham\": \"bone\"}") {
        Ok(obj) => {
            match obj {
                json::Object(o) => {
                    //println(fmt!("Payload! %?", o.get(& ~"ham")))
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

    let mut a:~[~str] = ~[];
    a.push(~"Hello");
    a.push(~" World");
    //println(a.concat());
}