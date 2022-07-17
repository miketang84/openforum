use anyhow::Result;
use bytes::Bytes;
use spin_sdk::redis_component;
use std::str::from_utf8;

/// A simple Spin Redis component.
#[redis_component]
fn on_message(message: Bytes) -> Result<()> {
    // the message is the data retreived from the redis channel
    println!("{}", from_utf8(&message)?);

    // the message is a JSON stringified data
    // deserialize it
    let json_obj = message.decode(); // message.deserialize();

    // check if the method is 'query' or 'event', or 'check_pair_list'
    // query: from the http gate, query the database
    // event: from the subxt event, pass the event data to the redis worker to handle
    // json_obj {
    //      reqid,
    //      method,
    //      path,
    //      data
    // }
    // The reqid and path field is a string that http gate sends to subxt to handle, should send it back to
    // redis worker to process consequently
    // The data field is the raw post request payload(usually is a url-encoded form data or a json
    // data)

    match method {
        "query" => {
            // handle query request
            // use postgres handle to query data from the postgres db
            let result = handle_query(reqid, path, data);

        }
        "event" => {

            let result = handle_event(reqid, path, data);

        }
        "check_pair_list" => {
            // ----- data transfer part -----

            // put result in path
            if path == "true" {
                // check pass, write this content to a cache
                let tmpdata = redis::get(&redis_addr, "tmp:cache:{reqid}");
                let _ = redis::set(&redis_addr, "cache:status:{reqid}", "true");
                let _ = redis::set(&redis_addr, "cache:{reqid}", tmpdata);
                // delete the tmp cache
                let _ = redis::del(&redis_addr, "tmp:cache:{reqid}");
            }
            else {
                // if not true, get which one is not equal, this info is in that data field '(id,
                // hash) is not right'
                let _ = redis::set(&redis_addr, "cache:status:{reqid}", "false");
                let _ = redis::set(&redis_addr, "cache:{reqid}", data);
                // clear another tmp cache key
                let _ = redis::del(&redis_addr, "tmp:cache:{reqid}");

            }
        }
        "update_pair_list" => {
            // ----- data transfer part -----
            // field data is: model:id

            // put result in path
            if path == "true" {
                // do nothing
            }
            else {
                // should handle, log it, why update the id-hash pair to onchain data failed
                log::it();

            }
        }
    }

    Ok(())
}

fn handle_query(reqid: String, path: String, data: String) -> Result<String> {
    match path {
        "/article" => {
            // ----- biz logic part -----

            // get the view of one article, the parameter is in 'data', in the format of url
            // encoded
            let params = data.decode();
            let article_id = params["id"];
            // construct a sql statement 
            let query_string = format!("select hash, id, title, content, author from article where id='{article_id}'");
            let query_results = pg::query(&pg_addr, query_string, &vec![]);

            // convert the raw vec[u8] to every rust struct filed, and convert the whole into a
            // rust struct vec, later we may find a gerneral type converter way
            let mut results: Vec<Article> = vec![];
            for row in query_results {
                let hash = String::from_utf8(col[0]);
                let id = String::from_utf8(col[1]);
                let title = String::from_utf8(col[2]);
                let content = String::from_utf8(col[3]);
                let author = String::from_utf8(col[4]);
                let article = Article {
                    hash,
                    id,
                    title,
                    content,
                    author,
                };
                results.push(article);
            }


        }
        "/articles" => {


        }

        "/comments" => {


        }
    }

    // ----- post process part -----

    // write this content to a tmp cache
    redis::set(&redis_addr, "tmp:cache:{reqid}", results.serialize());
    // construct a (id-hash) pair list
    let pair_list = results.then(|obj| (obj.id, obj.hash)).collect();
    // send this to the redis channel to subxt
    redis::publish(&redis_addr, "wasmengine2blockchain", pair_list.serialize())
        // and this flow ends, when next entrance of this redis handler, it execute another
        // method
}

fn handle_event(reqid: String, path: String, data: String) -> Result<String> {

    match path {
        "/article/new" => {

            let params = data.decode();
            let title = params["title"];
            let content = params["content"];

            let article_id = generate_id(); // uuid
            //let author = ...;

            // construct a struct
            let article = Article {
                "",
                id,
                title,
                content,
                author,
            };

            // should ensure the serialization way is determined.
            // and the field hash won't participate the serialization
            let hash = calc_hash(article.serialize());

            // construct a sql statement 
            let sql_string = format!("insert into article values (hash, article.id, article.title, article.content, article.author)");
            let execute_results = pg::execute(&pg_addr, sql_string, &vec![]);


        }
        "/article/edit" => {


        }
        "/article/delete" => {


        }
        "/comment/new" => {


        }

    }

    // if execute_results is ok
    // send the id hash pair (as a list) to the redis channel
    // construct a (id-hash) pair list
    let pair_list = Vec::new().push((id, hash));
    redis::publish(&redis_addr, "wasmengine2blockchain", pair_list.serialize());


    // we may wait update id-hash pair successfully, or not
    // here is not, then we won't wait for one more block interval time
    let _ = redis::set(&redis_addr, "cache:status:{reqid}", "true");
    let _ = redis::set(&redis_addr, "cache:{reqid}", article.id); // update the id back
}
