use std::process::id;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}
// ↑↓明示的に定義する
//　クロージャの所有権を奪い、Box<T>から値をムーブする
impl<F: FnOnce()> FnBox for F{
    fn call_box(self: Box<F>) {
        (*self)();
    }
}


type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        // 一つのreceiverを各workerで共有して、各workerではそのreceiverからタスクを受け取って、実行するみたいな実装にしたい
        // そのためには、receiverはMutexにする必要がある。かくworkerから同時アクセスできないように。
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // この共通のreceiverをworkerに渡して、senderからのメッセージを受け取れるようにする
            let receiver = Arc::clone(&receiver);
            let worker = Worker::new(id, receiver);
            threads.push(worker);
        }
        ThreadPool{
            threads,
            sender
        }
    }

    // thread::spawnが必要な引数を定義している。
    // トレイト境界として
    // Send・・・別のスレッドにクロージャを送るために必要
    // 'static・・・スレッドの実行にどれくらいかかるかわからないので、staticも必要
    // Fは、FnOnce() + Send + 'staticの三つ継承している感じ
    pub fn execute<F>(&self, f: F) where F: FnBox + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();

    }

}

// ThreadPoolがdropされる前に、保持しているworker（thread）の後処理をする必要がある
// 参考：https://doc.rust-jp.rs/book-ja/ch20-03-graceful-shutdown-and-cleanup.html
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.threads {
            // 各workerにloop停止のmessageを送っている
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("shutting down all workers");

        for worker in &mut self.threads {
            println!("shutting down worker: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                // joinメソッドは、所有権を奪う必要意があるので、このような作りにしている
                thread.join().unwrap();
            }
        }

    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||{
            // 永遠にループしているので、senderからmessageが送られてきたメッセージをreceiverが受け取って、
            // それぞれのworkerが処理している
            // jvmのthread poolもこんな感じのloop処理になっている
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("worker id: {}", id);
                        // Box<T>に格納したクロージャの所有権を奪って、実行したいが
                        // コンパイルすると怒られる。
                        // 実行しようとするクロージャのサイズがわからないからだ。
                        job.call_box();
                    },
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread)
        }
    }


}
