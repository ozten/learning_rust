extern mod extra;
extern mod http_client;



use extra::json;
use extra::net::url::Url;
use extra::net::url;
//use extra::serialize::{Decodable, Encodable};

use std::hashmap::HashMap;
use std::str;
use std::result::{Ok, Err};

use http_client::uv_http_request;
use http_client::StatusCode;

pub struct RepoResponse {
    rawJson: ~[~str]
}

impl RepoResponse {
    fn add(&mut self, data: ~str) {
        self.rawJson.push(data)
    }
}

/* https://github.com/huonw/isrustfastyet/blob/fce8777e289eb874a94242dbd07803248d320d12/mem/process.rs
#[deriving(Encodable)]
struct Repo {
    id: uint,
    name: ~str,
    full_name: ~str
}
*/

fn handleRepo(repo: & json::Json) {
    println(fmt!("decoded: %?\n", json::Decoder(copy *repo).read_str(~"name")));
}

fn readJson(json: json::Json) {
    match json {
        json::List(l) => {
            println(fmt!("Got %? items", l.len()));
            //for l.consume_iter().advance |repo| {
            for l.iter().advance |repo| {
                println(fmt!("repo=%?\n\n", repo));
                handleRepo(repo)


                        //x.find_copy(~"name");
                        //let d: Repo = Decodable::decode(&mut json::Decoder(json));
                        //println(fmt!("%?\n", repo.contains_key(~"git_commits_url")));


                    /*json::Json.Object(orepo) => {
                        println(fmt!("%?\n", orepo.contains_key(~"git_commits_url")));
                    }*/

                /*
                match repo {
                    json::Object(repo) => {

                    },
                    _ => {
                        fail!("We expected a list of objects");
                    }
                }*/
            }

            println("A list of Objects, perhaps")
        }
        _ => {
            println("ERROR: Unrecognized JSON format")
        }
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
            http_client::Status(s) =>
                match s {
                    // TODO wait... how did I break how match works here
                    // I should need the pattern guard.
                    StatusOK if s == StatusOK => {
                        println(fmt!("Status %?", s));
                        match json::from_str(res.rawJson.concat()) {
                            Ok(json) => {
                                readJson(json);
                            }
                            Err(e) => {
                                println(fmt!("Error parsing JSON %?", e));
                                fail!("Can't read JSON");
                            }
                        }
                    }
                    StatusFound if s == StatusFound => {
                        println(fmt!("Redirected? %?", s));
                    }
                    StatusUnknown => {
                        println(fmt!("hmmm status is unknown %?", s));
                        fail!("No JSON of Repositiories");
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