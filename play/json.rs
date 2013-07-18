extern mod extra;
extern mod std;

use std::hashmap::HashMap;

use extra::json::List;
use extra::json::Number;
use extra::json::Object;
use extra::json::String;
use extra::json::from_str;
use extra::json::Json;
use JsonError = extra::json::Error;

fn emptyObjectExample() {
    let o:~HashMap<~str, Json> = match from_str("{}") {
        Ok(Object(o)) => {
            o
        },
        Ok(_) => {
            fail!("Expected an object after parsing");
        },
        Err(e) => {
            println(fmt!("ERROR: %?", e));
            fail!("What, no parsing?");
        }
    };
    let mut counter = 0;
    for o.iter().advance() |k| {
        counter += 1;
    }
    println(fmt!("0 = %?", counter));
}

fn simpleObjectWithANumber() {
    match from_str("{\"foo\": 33}") {
        Ok(Object(o)) => {
            let map: ~HashMap<~str, Json> = o;
            println(fmt!("true = %?\n", map.contains_key(&~"foo")));
            if (map.contains_key(&~"foo")) {
                let foo = match map.find(&~"foo") {
                    Some( & Number(value)) =>  {
                            value
                    },
                    Some(_) => fail!("foo was wrong type"),
                    None => fail!("No element")
                };
                println(fmt!("33 = %?\n", foo))
            } else {
                println("No key foo")
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

fn listObjects() {
  let j:Result<Json,JsonError> = from_str("[{\"bar\":\"baz\", \"biz\":123}]");
  let l:List = match j {
    Ok(List(l)) => l,
    Ok(_) => fail!("Expected a list at the top level"),
    Err(e) => fail!(fmt!("Error: %?", e))
  };
  println(fmt!("item = %?", l.iter().advance(|i|{
    let item:Json = copy *i;
    match item {
        Object(o) => {
            println(fmt!("true = %?", o.contains_key(&~"bar")));
            //let barItem = ;
            let baz = match copy *o.get(&~"bar") {
                String(abaz) => abaz,
                _ => fail!("Expected bar property")
            };
            println(fmt!("bar = %?", baz));


            println(fmt!("true = %?", o.contains_key(&~"biz")));
            //let barItem = ;
            let biz = match copy *o.get(&~"biz") {
                Number(abiz) => abiz,
                _ => fail!("Expected biz property")
            };
            println(fmt!("biz = %?", biz));
        },
        _ => {
            fail!("Should be a list of objects, no?");
        }
    }
    true
  })));

}

fn main() {
    emptyObjectExample();
    simpleObjectWithANumber();
    listObjects();
}