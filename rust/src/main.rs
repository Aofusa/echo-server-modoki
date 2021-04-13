#![deny(warnings)]

mod storage;

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

use crate::storage::{Storage, Message};

#[derive(Deserialize, Serialize)]
struct Response {
    code: i32,
    result: String
}

async fn set(msg: Message, storage: Storage) -> Result<impl warp::Reply, warp::Rejection> {
    storage.storage.write().unwrap().insert("msg".to_string(), msg.msg);

    Ok(warp::reply::json(
        &Response { code: 0, result: "success".to_string() }
    ))
}

async fn get(storage: Storage) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let r = storage.storage.read().unwrap();

    for (key, value) in r.iter() {
        result.insert(key, value);
    }

    Ok(warp::reply::json(
        &result
    ))
}

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let storage = Storage::new();
    let storage_filter = warp::any().map(move || storage.clone());

    // GET /
    // TODO
    let root = warp::path::end()
        .and(warp::get())
        .map(|| {
            tracing::info!("GET root");
            let msg = Message { msg: "Hello, World at root!".to_string() };
            warp::reply::json(&msg)
        })
        .with(warp::trace::named("root"));

    // POST /echo
    let echo = warp::path("echo")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|message: Message| {
            tracing::info!("POST echo");
            warp::reply::json(&message)
        })
        .with(warp::trace::named("echo"));

    // POST /set
    let set = warp::path("set")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(storage_filter.clone())
        .and_then(set)
        .with(warp::trace::named("set"));

    // GET /get
    let get = warp::path("get")
        .and(warp::get())
        .and(storage_filter.clone())
        .and_then(get)
        .with(warp::trace::named("get"));

    let routes = root
        .or(echo)
        .or(set)
        .or(get)
        .with(warp::trace::request());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
