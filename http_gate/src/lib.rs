use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

const DEFAULT_INDEX: &str = "/";

/// A simple Spin HTTP component.
#[http_component]
fn http_gate(req: Request) -> Result<Response> {
    
    // print the query params, from the url 
    


    // print the post params in the post body in url encoded form


    // dispatch url, according to prefix: /r/, /w/
    // get: /r/...
    // post: /w/... 
    // we use html rendering or json? I suggest json. We can use an advance node to do server
    // rendering
    match req.headers().get("spin-path-info") {
        Some(path) => {
            if path.starts_with("/q/") {
                let method = "query";
                let rel_path = &path[2..];
                
                // We can do the unified authentication for some get actions here
                // TODO: 

                // construct a json, serialize it and send to a redis channel
                let json_to_send = json! {
                    "method": method,
                    "path": path.to_owned(),
                    "data": data,
                };

                // just push it into the redis channel
                redis::publish(&redis_addr, "channel_name", json_to_send.serialize());

            } else if path.starts_with("/w/") {
                let method = "write";
                let rel_path = &path[2..];

                // We can do the unified authentication for some get actions here
                // TODO: 

                // construct a json, serialize it and send to a redis channel
                let json_to_send = json! {
                    "method": method,
                    "path": path.to_owned(),
                    "data": data,
                };

                // just push it into the redis channel
                redis::publish(&redis_addr, "channel_name", json_to_send.serialize());


            } else {
                    // handle case of no fit url 

            }
        }
        None => {
            // handle no "spin-path-info" header

        }
    };




    //println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
}

/// Do query operation: get
fn query_action(path: &str, req: Request) -> Result<Vec<u8>> {

    // construct a json, serialize it and send to a redis channel
    let json_to_send = json! {
        "method": "get",
        "path": path.to_owned(),
        "data": data,
    };

    // just push it into the redis channel
    redis::publish(&redis_addr, "channel_name", json_to_send.serialize());


/*
    match path {
        &"/article/id" => {

        },
        &"/article/list" => {

        },
        &"/comment/id" => {

        },

    }
*/

}


/// Do post operation: add, update, delete
fn post_action(path: &str, req: Request) -> Result<String> {

    match path {
        &"/article/add" => {

        },
        &"/article/update" => {

        },
        &"/article/delete" => {

        },
        &"/comment/add" => {

        },

    }


}

