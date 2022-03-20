use std::time::Duration;
use async_std;
use async_std::task;

// async {}で非同期タスクは作れるが、実行する環境（ランタイム）がない
// // 非同期タスクを開始・実行・監視・完了させるために、非同期ランタイムであるtokioやasync-std crateを利用する
fn main() {
    let heavy_task = task::spawn(async {
        print_thread_info();
        std::thread::sleep(Duration::from_secs(10));
        println!("heavy!!!!");
    });

    let light_task = task::spawn(async {
        print_thread_info();
        std::thread::sleep(Duration::from_secs(3));
        println!("light!!!!");
    });

    let tasks = vec![heavy_task, light_task];
    println!("not async task!");

    // task::block_onは引数に渡した非同期タスクが完了するまでcurrent threadをブロックする
    task::block_on(async {
        for task in tasks {
            print_thread_info();
            task.await;
        }
    });
    println!("all task completed!");
}


fn print_thread_info() {
    let current_thread = std::thread::current();
    let thread_name = current_thread.name().unwrap_or("");
    let thread_id = current_thread.id();
    println!("{}-{:?}", thread_name, thread_id);
}
