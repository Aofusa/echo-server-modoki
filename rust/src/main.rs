#![deny(warnings)]

use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // GET /
    // TODO
    let root = warp::path::end()
        .and(warp::get())
        .map(|| {
            tracing::info!("GET root");
            "Hello, World at root!"
        })
        .with(warp::trace::named("root"));

    // POST /echo
    // TODO
    let echo = warp::path("echo")
        .and(warp::post())
        .map(|| {
            tracing::info!("POST echo");
            "Hello, World at echo!"
        })
        .with(warp::trace::named("echo"));

    // POST /set
    // TODO
    let set = warp::path("set")
        .and(warp::post())
        .map(|| {
            tracing::info!("POST set");
            "Hello, World at set!"
        })
        .with(warp::trace::named("set"));

    // GET /get
    // TODO
    let get = warp::path("get")
        .and(warp::get())
        .map(|| {
            tracing::info!("GET get");
            "Hello, World at get!"
        })
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
