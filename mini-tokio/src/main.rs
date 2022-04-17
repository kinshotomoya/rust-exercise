// TODO: 続きはここから、実際にtokioを作ってみる
// https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/async_in_depth#mini-tokio

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

fn main() {

    

}


type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
struct MiniTokio {
    tasks: VecDeque<Task>
}
