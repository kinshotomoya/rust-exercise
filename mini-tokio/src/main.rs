// TODO: 続きはここから、実際にtokioを作ってみる
// https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/async_in_depth#mini-tokio

use std::collections::VecDeque;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use futures::{select, task};
use futures::task::ArcWake;
use crossbeam::channel;

fn main() {
    let mut mini_tokio = MiniTokio::new();
    // このタスクが2番目に実行結果返る
    mini_tokio.spawn(async {

        let when = Instant::now() + Duration::from_secs(10);
        let future = Delay { when };

        let out = future.await;
    });


    // このタスクが最初に実行結果返る
    mini_tokio.spawn(async {

        let when = Instant::now() + Duration::from_secs(1);
        let future = Delay { when };

        let out = future.await;
    });


    mini_tokio.run();

}

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>
}

impl MiniTokio {
    fn new() -> MiniTokio {
        // std::channelはSyncを実装していない（非同期対応）ので、crossbeam crateを利用する
        let (sender, receiver) = channel::unbounded();
        MiniTokio {
            scheduled: receiver,
            sender
        }
    }

    // 引数で受けとったfutureをruntimeで共有のqueueにpushしている
    fn spawn<F>(&mut self, future: F)
        where F: Future<Output = ()> + Send + 'static {
        Task::spawn(future, &self.sender)
    }

    fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            println!("wwwww");
            task.pool();
        }
    }
}



// taskは紐づくWakerがwake()されたタイミングでチャネルの送信側を使ってtaskを送信する必要がある
struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }

    fn pool(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future: MutexGuard<Pin<Box<dyn Future<Output=()> + Send>>> = self.future.try_lock().unwrap();

        // futureをポーリングしている（Delay.poolが呼び出されている）
        future.as_mut().poll(&mut cx);
    }

    // 受け取ったfutureをTask構造体にラップしてチャネルに送信する
    // mini-tokio runした時点でこのtaskをreceiveしている
    fn spawn<F>(future: F, channel: &channel::Sender<Arc<Task>>) where F: Future<Output = ()> + Send + 'static {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: channel.clone()
        });

        channel.send(task);
    }
}

impl ArcWake for Task {
    // taskに紐づくwakerがwakeしたらこのメソッドが呼ばれる
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("mutt");
        arc_self.schedule();
    }
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // 現在のtaskに紐づくwakerを取得する
            let waker = cx.waker().clone();
            let when = self.when;

            std::thread::spawn(move || {
                let now = Instant::now();
                // まだ実行時間になっていない場合には、実行時間になるまでスレッドをスリープさせる
                if now < when {
                    std::thread::sleep(when - now);
                }
                // 実行時間になったらそのタスクに紐づくwakerにwakeupするように通知を送る
                // tokio runtime側でこの通知を受ける処理が必要
                println!("ssss");
                waker.wake();
            });

            Poll::Pending
        }
    }
}
