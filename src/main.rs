use axum::{
    body::Bytes,
    extract::{ContentLengthLimit, Extension, Path},
    handler::get,
    http::StatusCode,
    AddExtensionLayer, Router,
};
use rocksdb::DB;
use std::{iter::FromIterator, net::SocketAddr, sync::Arc};

type Shared = Arc<DB>;

async fn str_get(
    Path(key): Path<String>,
    Extension(state): Extension<Shared>,
) -> Result<Bytes, StatusCode> {
    match state.get(key) {
        Ok(Some(v)) => Ok(Bytes::from_iter(v)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

async fn str_set(
    Path(key): Path<String>,
    ContentLengthLimit(bytes): ContentLengthLimit<Bytes, { 1024 * 1024 }>,
    Extension(state): Extension<Shared>,
) {
    let _ = state.put(key, bytes);
}

#[tokio::main]
async fn main() {
    let path = "./data";
    let shared = Arc::new(DB::open_default(path).unwrap());

    let router = Router::new()
        .route("/:key", get(str_get).post(str_set))
        .layer(AddExtensionLayer::new(shared));

    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("ignored");

    println!("listening on : {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
