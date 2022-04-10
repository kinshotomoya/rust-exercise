use std::time::Duration;
use axum::{Json, Router};
use axum::routing::{get, post};
use crate::{IntoResponse, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::task;
use crate::hasher;
use crate::hasher::create_hash_from;

pub fn route() -> Router {
    Router::new()
        .route("/", get(root))
        .route("user", post(create_user))
        .route("/async_sync", get(async_sync))
        .route("/future", get(future))
}


async fn future() -> String {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(5));
        println!("sssssss");
    });
    String::from("com")
}


async fn async_sync() -> String {
    print_thread_info("async");
    // CPU高負荷なブロッキング処理は、spawn_blockingで別スレッドで処理させる
    let blocking_res = tokio::task::spawn_blocking(|| {
        print_thread_info("blocking");
        let hash = create_hash_from(String::from("hello world"));
        hash
    }).await;


    // ノンブロッキング処理はspawnで非同期所利用スレッドプールにタスク投げる
    // TODO: ノンブロッキングなhttp clientを使ってみようか
    // 各crateの比較：https://blog.logrocket.com/the-state-of-rust-http-clients/
    // hyper公式：https://github.com/hyperium/hyper
    tokio::task::spawn(async {

    });

    match blocking_res {
        Ok(hash) => hash,
        Err(_) => String::from("error")
    }
}

async fn root() -> &'static str{
    "hello world"
}

fn print_thread_info(prefix: &str) {
    let thread_info = std::thread::current();
    println!("{}-{}-{:?}", prefix, thread_info.name().unwrap_or(""), thread_info.id());
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1,
        user_name: payload.username
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
    struct User {
    id: u64,
    user_name: String,
}
