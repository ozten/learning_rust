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


fn main() {

    // http://ozten.com/psto/
    // https://api.github.com/repositories
    let u: Url = url::from_str("https://api.github.com/repositories").get();
    let mut request = uv_http_request(u);

    do request.begin |event| {
        let mut r:~[~str] = vec::with_capacity(1000000);
        //r.push(~"foo");
        //r.push(~"bar");
        match event {
            http_client::Error(e) => {
                println(fmt!("Ouch... error %?", e));
            },
            http_client::Status(s) => {
                println(fmt!("Status %?", s));
                println(r.concat());
                match json::from_str(r.concat()) {
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
                println(fmt!("Concating %?", str::from_bytes(data)));
                r.push(str::from_bytes(data));
                //r = r + str::from_bytes(data);
                //r.push_str(str::from_bytes(data));
                //r = r + str::from_bytes(data);
                println(fmt!("%?", r.len()));
                //println(fmt!("%?\n\n", r.concat()));

                // I think we need to know how many bytes and then
                // do the JSON parsing here...
            }
        }


    }
    println("Hardcoded json play");
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

    let mut a:~[~str] = ~[];
    a.push(~"Hello");
    a.push(~" World");
    println(a.concat());
}