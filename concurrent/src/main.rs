use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // rustでは標準で、1:1スレッドを適応している
    // つまり、OSスレッドとグリーンスレッドを1:1で対応させている。

    // 子スレッドはmainスレッドが閉じると実行途中でも閉じられる
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("new thread: {}", i);
                thread::sleep(Duration::from_micros(1));
            }
        });

        for i in 1..100 {
            println!("main thread: {}", i);
        }

        // joinすることでhandleに格納したスレッドが終了するまで次の処理に行かない
        handle.join().unwrap();
    }

    {

        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            // これはできない
            // mainスレッドで作成したvector(v)を子スレッドで利用するためにはmoveをつける必要がある
            // => vの所有権を子スレッドに渡す必要がある！！
            // panic発生
            println!("here is a vector {:?}", v);
        });

        handle.join().unwrap();
    }

    // なぜなら
    // 以下のように、所有権を渡さずに参照のみを渡している場合、子スレッドが終了する前に
    // mainスレッドの方でvがドロップされてしまう可能性があるから。
    // ↓こんな感じで
    // {
    //     let v = vec![1, 2, 3];
    //
    //     let handle = thread::spawn(|| {
    //         // これはできない
    //         // mainスレッドで作成したvector(v)を子スレッドで利用するためにはmoveをつける必要がある
    //         // => vの所有権を子スレッドに渡す必要がある！！
    //         // panic発生
    //         println!("here is a vector {:?}", v);
    //     });
    //
    //     drop(v);
    //
    //     handle.join().unwrap();
    //
    // }

    {
        // チャンネルを使って、スレッド間でメッセージをやり取りする

        let (tx, rx) = mpsc::channel();

        // 転送機をクローンして、複数転送機 -> 受信機にメッセージを送ることができる
        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let vec = vec![
                String::from("hi"),
                String::from("yo"),
                String::from("hey"),
                String::from("????")
            ];

            for v in vec {
                tx.send(v).unwrap();
                thread::sleep(Duration::from_micros(10))
            }
        });


        thread::spawn(move || {
            let vec = vec![
                String::from("i"),
                String::from("am"),
                String::from("kinsho"),
                String::from("tomoya")
            ];

            for v in vec {
                tx1.send(v).unwrap();
                thread::sleep(Duration::from_micros(10))
            }
        });

        // recvはmainスレッドをブロックする
        for r in rx {
            println!("got {}", r);
        }

        println!("finish");

    }


}
