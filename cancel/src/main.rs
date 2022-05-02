use std::time::Duration;
use tokio::sync::oneshot;

async fn some_do() -> String {
    std::thread::sleep(Duration::from_secs(5));
    println!("some do");
    "".to_string()
}

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {

        tokio::select! {
            val = some_do() => {
                println!("some do in select!");
                let _ = tx1.send(val);
            }
            // rx1がドロップされた時にcloseメッセージが送られるので
            // ここを通るようになる
            _ = tx1.closed() => {
                println!("close");
            }
        }

    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // 最初に完了した方が実行される
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }

}
