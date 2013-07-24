extern mod extra;
extern mod http_client;
extern mod sqlite;

extern mod link_header;

use std::hashmap::HashMap;
use std::str;
use std::result::{Ok, Err};

use extra::json;
use extra::json::{Object, List, String, Number};
use extra::net::url::Url;
use extra::net::url;

use http_client::uv_http_request;
use http_client::StatusCode;
use sqlite::open;

use link_header::*;
pub struct RepoResponse {
    rawJson: ~[~str],
    inLinkField: bool
}

impl RepoResponse {
    fn add(&mut self, data: ~str) {
        self.rawJson.push(data)
    }
}

fn s(key:~str, j:~HashMap<~str, json::Json>) -> ~str {
    match copy *j.get(&key) {
        String(value) =>  value,
        _ => fail!("foo was wrong type " + key)
    }
}

fn n(key:~str, j:~HashMap<~str, json::Json>) -> float {
    match copy *j.get(&key) {
        Number(value) =>  value,
        _ => fail!("foo was wrong type")
    }
}

fn handleRepo(repo: json::Json) {
    match repo {
        Object(o) => {
            println(fmt!("repository_id = %?", n(~"id", copy o)));

            if (o.contains_key(& ~"name")) {
                let name = match copy *o.get(&~"full_name") {
                    String(value) =>  value,
                    _ => fail!("foo was wrong type")
                };
                println(fmt!("repository_name %?", name));
            }

            println(fmt!("html_url = %?", s(~"html_url", o)));
        },
        _ => fail!("Why you no Repo?")
    }
}

fn readJson(json: json::Json) {
    match json {
        json::List(l) => {
            println(fmt!("Got %? items", l.len()));
            //for l.consume_iter().advance |repo| {
            for l.iter().advance |repo| {
                //println(fmt!("repo=%?\n\n", repo));
                handleRepo(copy *repo);
            }
            println("A list of Objects, perhaps")
        }
        _ => {
            println("ERROR: Unrecognized JSON format")
        }
    }
}

// Task
// Given a starting https://api.github.com/repositories URL
// This task downloads the metadata, figures out the next
// api_url and stores these in the database. it then starts
// over with the new api_url
fn slurp_repos(api_url:&str) {
    // https://api.github.com/repositories
    // http://ozten.com/random/sample.json
    let u: Url = url::from_str(api_url).get();
    let mut options:HashMap<~str, ~str> = HashMap::new();


    options.insert(~"User-Agent",
                   ~"curl/7.21.4 (universal-apple-darwin11.0) libcurl/7.21.4 OpenSSL/0.9.8x zlib/1.2.5");
    options.insert(~"Accept", ~"*/*");
    // ~"curl/7.21.4 (universal-apple-darwin11.0) libcurl/7.21.4 OpenSSL/0.9.8x zlib/1.2.5"


    let mut request = uv_http_request(u, options);

    let res = @mut RepoResponse{rawJson: ~[], inLinkField: false};

    do request.begin |event| {
        match event {
            http_client::Error(e) => {
                println(fmt!("Ouch... error %?", e));
            },
            http_client::Status(s) => match s {
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
            http_client::HeaderField(field) => {
                let hField = str::from_bytes(field.take());
                match hField {
                    ~"link" | ~"Link" => {
                        res.inLinkField = true;
                        println("We found link");
                    },
                    _ => ()
                }
            },
            http_client::HeaderValue(field) => {
                if (res.inLinkField) {
                    res.inLinkField = false;
                    let hValue = str::from_bytes(field.take());
                    println("Queing up next page from ");
                    let link:@~str = link_header::parse(hValue);
                    //println(*link.replace("api.github.com", "localhost:8002"));
                    println(link.replace("api.github.com", "localhost:8002"));

                }

            },
            http_client::Payload(p) => {
                let data = p.take();
                res.rawJson.push(str::from_bytes(data));
            }
        }
    }
}