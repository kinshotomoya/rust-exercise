fn main() {

    {

        let n = 100i64;

        let o1 = Some(&n);

        let o2 = o1.map(|x| *x + 456);

        println!("{} bytes", std::mem::size_of::<Option<i64>>());
        // => 16 bytes
        // つまり、Option<i64>のためには、16bytesのメモリが必要
        // i64は8bytesなので、タグのために余分に8bytesが必要になる


        println!("{} bytes", std::mem::size_of::<Option<&i64>>());
        // => 8 bytes
        // 一方でこちら、Option<&i64>のメモリサイズを確認すると、8bytesになっている
        // タグが無くなった
        // 元のi64のbytesだけを保持している

    }


}
