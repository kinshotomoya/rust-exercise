use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
// use tokio_util::codec;
// use tokio_util::codec::{BytesCodec, Decoder};
use serde::{Deserialize, Serialize};
use signal_hook::consts::SIGINT;
use signal_hook::iterator::backend::PollResult::Signal;
use signal_hook::iterator::{Signals, SignalsInfo};
use tokio::signal::ctrl_c;

// tokioを使ってweb serverを実装
// 参考：https://github.com/tokio-rs/tokio/blob/master/examples/echo.rs
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//
//     let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await?;
//
//     loop {
//         let (mut tcp_stream, _) = listener.accept().await?;
//         // 別スレッドをたててread writeの処理をしている
//         // こうしないと, あるリクエストの処理が終わるまで別リクエストの処理ができない
//         tokio::spawn(async move {
//             let mut buf = vec![0; 1024];
//             loop {
//                 let n = tcp_stream.read(&mut buf).await.expect("faile to read data from socket");
//                 println!("{:?}", buf);
//
//                 if n == 0 {
//                     return ;
//                 }
//
//                 tcp_stream.write_all(&buf[0..n]).await.expect("");
//             }
//         });
//
//     }
// }


// axum sample
// 参考： https://github.com/tokio-rs/axum/blob/main/examples/readme/src/main.rs
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("user", post(create_user));

    let socket = SocketAddr::from(([127, 0, 0, 1], 8080));

    // TODO: シグナルハンドリング
    // 参考：https://rust-cli.github.io/book/in-depth/signals.html
    // ctrlc crateを使うとCTRL + Cのシグナルと受け取ることができる
    // ただこれだとctrl cのシグナルしかハンドリングできない
    // ctrlc::set_handler(|| {
    //     println!("receive!!!");
    //     // ↓こんな感じでプロセス殺せる
    //     std::process::exit(1)
    // }).expect("fail");


    let mut signals: SignalsInfo = Signals::new(&[SIGINT]).expect("");
    // mainスレッドで↓このようにシグナル待ちをしてしまうと、後続のweb serverの立ち上げができなくなるので
    // シグナル処理は別スレッドで行う必要がある
    for sig in signals.forever() {
        println!("sss");
    }

    axum::Server::bind(&socket).serve(app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str{
    "hello world"
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

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    user_name: String,
}
