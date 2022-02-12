use std::sync::{Mutex, MutexGuard, Arc};
use std::thread;
use std::rc::Rc;
use std::time::Duration;

fn main() {
    {
        // Mutex: 一つだけのスレッドにのみ指定したデータのアクセス許可しないを実現するもの
        let m = Mutex::new(5);

        {
            // lockでロックを獲得している
            let mut num: MutexGuard<i32> = m.lock().unwrap();
            // MutexGuardはスマートポインタなので、Derefを実装している
            // なので、*numとするだけで、内部実データを取得している
            *num = 6;

            // また、Dropも実装しているので、
            // このスコープを抜けた時点で「ロックを解除する」といった処理がなされる
            // コード読むと↓のような実装をしている（ロック解除してそう）
            // #[stable(feature = "rust1", since = "1.0.0")]
            // impl<T: ?Sized> Drop for MutexGuard<'_, T> {
            //     #[inline]
            //     fn drop(&mut self) {
            //         unsafe {
            //             self.lock.poison.done(&self.poison);
            //             self.lock.inner.raw_unlock();
            //         }
            //     }
            // }
        }
        println!("{:?}", m);
    }

    {
        // Rc<T>はスレッドセーフではない。。。
        // なので、複数スレッド間で複数所有権を保持したい場合は、Rcは使えない。。。
        // 参照数を管理しているが、例えば3スレッドから同時に+1などとされると、ほんまは+3されたいが、+1しかされないなど
        // 不正な挙動になってしまう。。。
        // Rcの代わりにArcを使う
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 1..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }

        println!("{:?}", *counter)

    }

    {
        // マルチスレッドでのデッドロックを実現してみる

        // とりあえず、Mutex::newで一つのスレッドからのみアクセス可能と定義している
        // Arcは、複数のスレッド（thread1, thread2）に所有権を渡す必要があるので。
        let value1 = Arc::new(Mutex::new(1));
        let value2 = Arc::new(Mutex::new(2));

        // thread1内にvalue1, value2の所有権を渡すために
        // value1, value2へのポインターを複製している&参照数をカウント
        let value1_for_thread1 = Arc::clone(&value1);
        let value2_for_thread1 = Arc::clone(&value2);

        // thread2内にvalue1, value2の所有権を渡すために
        // value1, value2へのポインターを複製している&参照数をカウント
        let value1_for_thread2 = Arc::clone(&value1);
        let value2_for_thread2 = Arc::clone(&value2);

        let thread1 = thread::spawn(move || {
            println!("thread 1");
            // value1のlockを取得
            let v1 = value1_for_thread1.lock().unwrap();
            println!("value1 content is {} in thread 1", *v1);
            thread::sleep(Duration::from_secs(3));
            println!("waiting for getting value2 in thread 1");
            // value2のlockを取得しようとするが、thread2でvalue2がロックされている
            let v2 = value2_for_thread1.lock().unwrap();
            println!("value2 content is {} in thread 1", *v2);
            // このスコープを抜けないと、value1のロックを離さないのでデッドロック発生
        });

        let thread2 = thread::spawn(move || {
            println!("thread 2");
            // value2のlockを取得
            let v2 = value2_for_thread2.lock().unwrap();
            println!("value2 content is {} in thread 2", *v2);
            thread::sleep(Duration::from_secs(3));
            println!("waiting for getting value1 in thread 2");
            // value1のlockを取得しようとするが、thread1でvalue1がロックされている
            let v1 = value1_for_thread2.lock().unwrap();
            println!("value1 content is {} in thread 2", *v1);
            // このスコープを抜けないと、value2のロックを離さないのでデッドロック発生
        });

        for t in vec![thread1, thread2] {
            t.join().unwrap();
        }

    }

}
