use std::fs::read_to_string;
use std::time::Duration;
use tokio::io;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main(){


    println!("start");
    println!("{:?}", std::thread::current().name());
    // rustはデフォルトで非同期タスクを実行するruntimeを備えていないのでtokioなどのcrateを利用する必要がある
    // async{}だけだと、awaitしないと実行されない
    // main threadで↓も実行される
    let task =  async {
        println!("{:?}", std::thread::current().name());
        std::thread::sleep(Duration::from_secs(10));
        println!("async1");
    };

    // tokio::spawnすると別thread poolのworkerを利用する
    // awaitしなくてもworkerが実行している
    // 戻り値が欲しい場合は、awaitする必要がある
    let tokio_task_1 = tokio::spawn(async {
        println!("{:?}", std::thread::current().name());
        std::thread::sleep(Duration::from_secs(8));
        println!("tokio-async1");
        "tokio-async1"
    });

    println!("do something");

    let task2 = async {
        println!("async2");
    };

    let tokio_task_2 = tokio::spawn(async {
        println!("{:?}", std::thread::current().name());
        std::thread::sleep(Duration::from_secs(4));
        println!("tokio-async2");
        "tokio-async2"
    });

    let task3 = tokio::spawn(async {
        println!("async3");
    });

    std::thread::sleep(Duration::from_secs(10));

    // awaitすることによって、main threadがqueueから非同期タスクをpollする
    task.await;
    task2.await;
    // 別workerに投げた処理の戻り値はawaitするか、複数ある場合はtokio::join!で取得できる
    let (result_1, result_2) = tokio::join!(tokio_task_1, tokio_task_2);
    println!("{:?}", result_1);
    println!("{:?}", result_2);
}
