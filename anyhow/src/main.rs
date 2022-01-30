use std::fmt::format;
use std::fs::File;
use std::io;
use anyhow::Context;
// use anyhow::Result;

fn main() {


    {
        fn get_file(file_name: &str) -> Result<File, io::Error> {
            // この?は以下記法と同じ
            // つまり、Err起こした時点でResult型をreturnする
            // match File::open(file_name) {
            //     Ok(f) => f,
            //     Err(e) => return Err(e)
            // }

            let f = File::open(file_name)?;
            Ok(f)
        }

        get_file("sample.txt");
    }

    // anyhowを使う
    {
        use anyhow::Result;
        // anyhow::Resultになっている
        // これは、Result<(), anyhow::Errr>
        fn get_file(file_name: &str) -> Result<File> {
            // context情報など付与できるようになっている！
            // エラー文がリッチになる
            let f = File::open(file_name).with_context(|| format!("cdcdcdcdcdcd"))?;
            Ok(f)
        }

        get_file("sample.txt").unwrap();
    }

}
