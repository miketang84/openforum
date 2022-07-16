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
    //println!("{:?}", req.headers());


    // print the post params in the post body in url encoded form


    // dispatch url, according to prefix: /r/, /w/
    // get: /r/...
    // post: /w/... 
    // we use html rendering or json? I suggest json. We can use an advance node to do server
    // rendering
    let mut path = String::new();
    match req.headers().get("spin-path-info") {
        Some(path) => {
            path = path;
        },
        None => {
            // handle case of no spin-path-info in headers
        }
    }

    let mut method = String::new();
    let mut data = String::new();
    // we'd better use req.method() as our divider
    //match req.headers().get("spin-path-info") {
    match req.method() {
        &Method::GET => {
            method = "query".to_string();

            // In get mode: data is the url params
            data = ...
        }
        &Method::POST => {
            method = "write";

            // In post mode: data is the body content of the request
            data = ...
        }
        _ => {
            // handle cases of other directives

        }
    };

    // We can do the unified authentication for some get actions here
    // TODO: 

    // use a unique way to generate a reqid
    let reqid = Uuid::new();

    // construct a json, serialize it and send to a redis channel
    let json_to_send = json! {
        "reqid": reqid.clone(),
        "method": method,
        "path": rel_path.to_owned(),
        "data": data,
    };

    // just push it into the redis channel
    redis::publish(&redis_addr, "wasmengine2blockchain", json_to_send.serialize());


    loop {
        let mut loop_count = 1;
        // loop the redis cache key of this procedure request
        let result = redis::get(&redis_addr, &format!("reqid:{reqid}"));
        match result {
            Some(raw_result) => {
                // Now we get the raw serialized result from worker, we suppose it use
                // JSON spec to serialized it, so we can directly pass it to the back
                // to user response body.

                // deserialize the raw result? Need it?

                // clear the redis cache key of the worker result
                let _ = redis::del(&redis_addr, &format!("reqid:{reqid}"));


                // jump out this loop, and return the response to user
                return Ok(http::Response::builder()
                          .status(200)
                          .header("foo", "bar")
                          .body(Some(raw_result))?);

            }
            None => {
                // after 6 seconds, timeout
                if loop_count < 600 {
                    // if not get the result, sleep for a little period
                    std::sleep(10ms);
                    loop_count += 1;
                }
                else {
                    // timeout handler, use which http status code?
                    return Ok(http::Response::builder()
                              .status(500)
                              .body(Some("No data".into()))?);
                }
            }
        }
    }


}

