// TODO: 続きはここから、実際にtokioを作ってみる
// https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/async_in_depth#mini-tokio

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use futures::task;

fn main() {



}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

struct MiniTokio {
    tasks: VecDeque<Task>
}

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new()
        }
    }

    // 引数で受けとったfutureをruntimeで共有のqueueにpushしている
    fn spawn<F>(&mut self, future: F)
        where F: Future<Output = ()> + Send + 'static {
        self.tasks.push_back(Box::pin(future));
    }

    pub fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }

    }
}
