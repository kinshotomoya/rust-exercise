// このmodを定義することでmainのmodule treeに登録している感じ
mod signal_handling;
mod server;
mod route;
mod hasher;
use std::collections::HashMap;
use std::fmt::format;
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
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::backend::PollResult::Signal;
use signal_hook::iterator::{Signals, SignalsInfo};
use signal_hook::iterator::exfiltrator::WithOrigin;
use tokio::signal::ctrl_c;
use tokio::signal::unix::signal;
use crate::signal_handling::Command;

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
    let socket = SocketAddr::from(([127, 0, 0, 1], 8080));
    // 参考：https://rust-cli.github.io/book/in-depth/signals.html
    // 方法1
    // ctrlc crateを使うとCTRL + Cのシグナルと受け取ることができる
    // ただこれだとctrl cのシグナルしかハンドリングできない
    // ctrlc::set_handler(|| {
    //     println!("receive!!!");
    //     // ↓こんな感じでプロセス殺せる
    //     std::process::exit(1)
    // }).expect("fail");


    // 方法2
    // let mut signals: SignalsInfo = Signals::new(&[SIGINT]).expect("");
    // mainスレッドで↓このようにシグナル待ちをしてしまうと、後続のweb serverの立ち上げができなくなるので
    // シグナル処理は別スレッドで行う必要がある
    // thread::spawn(move || {
    //     for sig in signals.forever() {
    //         println!("sss");
    //         std::process::exit(1);
    //     }
    // });

    // 方法3
    // channelを使って処理する
    let (tx, rx) = tokio::sync::oneshot::channel::<Command>();

    // tokio::spawnは別スレッドを作成しているわけではない
    // 非同期タスクを作って、同一スレッドで渡した処理をさせている
    let signal_handle_thread = tokio::spawn(async move {
        signal_handling::signal_handling(tx)
    });

    // awaitしないとserver起動しない
    // run_serverメソッドはasyncになっていてmainスレッドで待ってあげないと、下の処理に進んでしまう
    server::run_server(socket, rx).await;
    // signal handling threadがちゃんと終わってからmain threadを終わらせるために必要
    // thread::spawnでいう thread.join()と同じ
    signal_handle_thread.await;

    // TODO:
    //  async awaitをちゃんと理解する
    //  1. ログの設定
    //  2. configの設定
    //  3. httpClient（connection pool）で外部APIを叩けるように
    //  4. 別スレッドでredisサーバを叩くように
}
