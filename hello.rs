extern mod extra;
extern mod http_client;

use extra::net::url::Url;
use extra::net::url;

use http_client::uv_http_request;

fn main() {

    debug!("debug here");
    // http://ozten.com/psto/
    // https://api.github.com/repositories
    let u: Url = url::from_str(~"https://api.github.com/repositories").get();
    let mut request = uv_http_request(u);
    do request.begin |event| {
        match event {
    	    http_client::Error(e) => {
    	        println(fmt!("Ouch... error %?", e));
    	    },
    	    http_client::Status(s) => {
    	        println(fmt!("Status %?", s));
    	    },
    	    http_client::Payload(p) => {
    	        let data = p.take();
    	        //println(fmt!("Payload! %?", str::from_bytes(data)));
     	    }
	    }
    }
}