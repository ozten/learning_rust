extern mod extra;
extern mod std;

use std::hashmap::HashMap;

use extra::json::Object;
use extra::json::Number;
use extra::json::String;
use extra::json::from_str;
use extra::json::Json;

fn main() {
    match from_str("{\"foo\": 33}") {
        Ok(Object(o)) => {
            let map: ~HashMap<~str, Json> = o;
            println(fmt!("%?\n", map.contains_key(&~"foo")));
            if (map.contains_key(&~"foo")) {
                let foo = match map.find(&~"foo") {
                    Some( & Number(value)) =>  {

                            println(fmt!("%?\n", value));
                            //value.take_unwrap().to_str()
                            ~"huh"
                    },
                    Some(_) => fail!("foo was wrong type"),
                    None => ~"Missing"
                };

            } else {
                println("No key foo");
            }
        },
        Ok(_) => {
            println(fmt!("Unexpected JSON value"));
        },
        Err(e) => {
            println(fmt!("ERROR: %?\n", e));
        }
    }

}