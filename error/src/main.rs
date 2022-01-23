use std::fs::File;
use std::io::Read;

fn main() {
    {
        let v = vec![1, 2, 3];
        // v[999];

        // 以下のようなエラーが発生する
        // RUST_BACKTRACE=1 cargo run
        // Finished dev [unoptimized + debuginfo] target(s) in 0.00s
        // Running `target/debug/error`
        // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 999', src/main.rs:3:5
        // stack backtrace:
        // 0: rust_begin_unwind
        // at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
        // 1: core::panicking::panic_fmt
        // at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
        // 2: core::panicking::panic_bounds_check
        // at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:76:5
        // 3: <usize as core::slice::index::SliceIndex<[T]>>::index
        // at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/slice/index.rs:184:10
        // 4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
        // at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/slice/index.rs:15:9
        // 5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
        // at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/vec/mod.rs:2496:9
        // 6: error::main
        // at ./src/main.rs:3:5
        // 7: core::ops::function::FnOnce::call_once
        // at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/ops/function.rs:227:5
        // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
        // ➜  error git:(master) ✗
    }

    // {
    //     // Result型を使って、エラーから回復する
    //     use std::fs::File;
    //
    //     // open関数は、Result<File>を返す関数である
    //     let file = File::open("fvfv.txt");
    //
    //     let fileResult = match file {
    //         // Result型は、OkとErrの列挙型
    //         Ok(file) => file,
    //         Err(error) => {
    //             panic!("problem!!!!!!")
    //         }
    //     };
    // }

    {
        use std::fs::File;
        use std::io::ErrorKind;

        let file = File::open("text.txt");
        match file {
            Ok(_file) => _file,
            // error.kindで、ErrorKind型を返す
            Err(ref _error) if _error.kind() == ErrorKind::NotFound => {
                println!("{}", _error);
                // refは、_errorをガード式にムーブされないようにしている
                // refと&はどちらも参照を表すようになっているが、
                // &・・・参照にマッチする
                // ref・・・その値にマッチする
                // 18章で詳細
                match File::create("kinsho.txt") {
                    Ok(file) => file,
                    Err(_error) => panic!("ファイル作成しようとしたがエラー吐いてしまった {:?}", _error)
                }
            },
            Err(_error) => panic!("openできなかった {:?}", _error)
        };
    }

    {
        // unwrapを使ってみる
        use std::fs::File;
        let file = File::open("s");
        // unwrapは、Errの場合panic起こす
        // file.unwrap();
        // expectもpanic起こすが、messageを指定できる
        // まあ、そもそも↑のfile.unwrap();でfileをムーブしているので、file自体は無効になっている
        // file.expect("");

    }

    {
        use std::io;
        use std::fs::File;
        fn read_file_name() -> Result<String, io::Error> {
            let file = File::open("kinsho.txt");

            let mut f = match file {
                Ok(_file) => _file,
                Err(_error) => return Err(_error),
            };

            let mut stringBuffer = String::new();
            match f.read_to_string(&mut stringBuffer) {
                Ok(size) => Ok(stringBuffer),
                Err(error) => Err(error)
            }
        }
    }

    {
        // ↑では、match式を多用して早期returnなどを実現していたが、もっと簡単な方法がある
        // ?を使うことで、Okの場合は　Okを返し、Errの場合はreturn Errで早期returnを実現できている

        // ?は、Result型を返す関数にのみ使える

        use std::io;
        fn read_file_name() -> Result<String, io::Error> {
            let mut stringBuffer = String::new();
            File::open("kinsho.txt")?.read_to_string(&mut stringBuffer)?;
            Ok(stringBuffer)
        }
    }

    {
        pub struct Guess {
            value: u32
        }

        impl Guess {
            pub fn new(value: u32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("nooooooooooooooo: {}", value);
                }
                Guess {
                    value
                }
            }

            pub fn value(&self) -> u32 {
                self.value
            }

        }


    }


    {
        let s = &(String::from("A"), 1);
        // refをつけることで、値の参照を束縛すると定義する
        let &(ref a, b) = s;

    }

    // {
    //     let robot_name = Some(String::from("robo!"));
    //
    //     match robot_name {
    //         // robot_nameの所有権は、すでに↓のnameにムーブしている
    //         Some(name) => println!("{}", name),
    //         None => println!("no name")
    //     }
    //     // なので、↓ではrobot_nameは使えない（削除されている）
    //     println!("{:?}", robot_name);
    //
    // }

    {
        // こんな時に、refを使う

        let robot_name = Some(String::from("robo"));
        match robot_name {
            // refを使うとrobot_nameのデータへの参照をとってきている
            Some(ref name) => println!("{}", name),
            None => println!("no name")
        }
        println!("{:?}", robot_name);

    }


}
