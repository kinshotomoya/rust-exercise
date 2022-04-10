use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::mpsc::Sender;
use crate::Command::{Get, Set};

#[tokio::main]
// このtokio::mainはmacroになっており、ランタイムの初期化と非同期処理の実行が行われる
pub async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let set_tx = tx.clone();

    let task1 = tokio::spawn(async move {
        // NOTE: redis serverからの結果を受け取るためにそれ用のチャネルを作成する
        let (one_tx, one_rx) = tokio::sync::oneshot::channel();
        // NOTE: menagerタスク側がserverから受け取ったレスポンスをone_txを通して送る
        tx.send(Command::Get {key: String::from("hello"), sender: one_tx}).await;
        // NOTE: one_tx（managerタスク）からのメッセージを待つ
        let res = one_rx.await;
        println!("GOT {:?}", res);
    });

    let task2 = tokio::spawn(async move {
        // NOTE: redis serverからの結果を受け取るためにそれ用のチャネルを作成する
        let (one_tx, one_rx) = tokio::sync::oneshot::channel();
        // NOTE: menagerタスク側がserverから受け取ったレスポンスをone_txを通して送る
        set_tx.send(Command::Set {key: String::from("hello"), value: "world".into(), sender: one_tx}).await;
        // NOTE: one_tx（managerタスク）からのメッセージを待つ
        let res = one_rx.await;
        println!("GOT {:?}", res);
    });

    let manager = tokio::spawn(async move {
        // mini-redis アドレスへのコネクションを開く
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get {key, sender} => {
                    let res = client.get(&key).await;
                    let _ = sender.send(res);
                },
                Set {key, value, sender} => {
                    let res = client.set(&key, value.into()).await;
                    let _ = sender.send(res);
                }
            }
        }
    });
    task1.await.unwrap();
    task2.await.unwrap();
    manager.await.unwrap();
}

enum Command {
    Set {
        key: String,
        value: Vec<u8>,
        sender: tokio::sync::oneshot::Sender<mini_redis::Result<()>>
    },
    Get {
        key: String,
        sender: tokio::sync::oneshot::Sender<mini_redis::Result<Option<Bytes>>>
    }
}
