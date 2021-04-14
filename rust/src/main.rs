#![deny(warnings)]

mod storage;

use serde_derive::{Deserialize, Serialize};

use actix::prelude::*;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

use crate::storage::{MsgActor, Msg};

#[derive(Deserialize, Serialize)]
struct ResponseSet {
    code: i32,
    result: String,
}

#[derive(Deserialize, Serialize)]
struct ResponseGet {
    msg: String,
}

#[derive(Deserialize, Serialize)]
struct RequestMsg {
    msg: String,
}

async fn set(msg: RequestMsg, storage: Addr<MsgActor>) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = storage.send(Msg::Set(msg.msg)).await;

    Ok(warp::reply::json(
        &ResponseSet { code: 0, result: "success".to_string() }
    ))
}

async fn get(storage: Addr<MsgActor>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(
        &ResponseGet { msg: storage.send(Msg::Get).await.unwrap() }
    ))
}

#[actix::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let storage = MsgActor { msg: "initialize msg".to_string() }.start();
    let storage_filter = warp::any().map(move || storage.clone());

    // GET /
    let root = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("../public/static/api.json"))
        .with(warp::trace::named("root"));

    // POST /echo
    let echo = warp::path("echo")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|message: RequestMsg| {
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
