mod connection;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use mini_redis::{Command, Frame};
use mini_redis::Command::{Set, Get};
use tokio::net::{TcpListener, TcpStream};

type DbType = Arc<Mutex<HashMap<String, Vec<u8>>>>;

// redis serverを作成
#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // NOTE: 複数スレッド内で同じオブジェクトを参照するためにArcでポインターを複製する必要あり
    let db: DbType = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // NOTE: 非同期タスクを作成
        // タスクはスケジューラによって管理される実行単位
        // rustのタスクは非常に軽量
        let dd = db.clone();
        tokio::spawn(async move {
            process(socket, dd).await;
        });
    }
}

// redisコマンドを受け付けて、db（メモリ、hashMap）に保存する」
async fn process(socket: TcpStream, db_mutext: DbType) {
    let mut connection = mini_redis::Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // NOTE: tokioのランタイムは.awaitのところでタスクを別スレッドに移す可能性があるが
                // 今回で言うとdb（MutexGuard）はSend traitを実装していないので、別スレッドに移すことができない
                // なのでコンパイラエラーになってしまう
                let mut db = db_mutext.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db_mutext.lock().unwrap();
                if let Some(va) = db.get(cmd.key()) {
                    Frame::Bulk(va.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("sss{:?}", cmd)
        };

        connection.write_frame(&response).await.unwrap();
    };

}
