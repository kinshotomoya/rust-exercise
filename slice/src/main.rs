fn main() {
    {

        fn first_word(word: &String) -> &str {
            let word_byte = word.as_bytes();

            for (i, &item) in word_byte.iter().enumerate() {
                if item == b' ' {
                    return &word[0..i];
                }
            }

            &word[..]
        }

        let mut word: String = String::from("hello world");
        let result = first_word(&word);

        // word.clear(); first_word(&word)で不変借用をしているので、word.clear()と可変借用はできない
        // とコンパイルエラーが出る
        println!("{}", result)

    }

    {

        let s = String::from("hello world");
        let hello = &s[1..];
        let world = &s[6..11];

        println!("{} {}", hello, world)


    }

    {
        let name = String::from("tomoya kinsho");
        let slice_name: &str = &name[..];
        let slice_name1: &str = &slice_name[..1];
        let slice_name2: &str = &slice_name[2..];
        // :pで表されるのは、Stringと&strで違う
        // Stringの場合・・・ヒープ上のString構造体の先頭アドレス
        // &strの場合・・・ヒープ上のUTF-8バイト列の先頭アドレス
        // なので、$nameは違う値になる
        // またslice_name2も違う値になる
        // &strはバイト列の先頭アドレスのポインタを返すので、slice_name2は他と比べて先頭バイト列が違うので！
        println!("{:p}", &name);
        println!("{:p}", slice_name);
        println!("{:p}", slice_name1);
        println!("{:p}", slice_name2);

    }

    {
        // 文字列はバイナリに直接埋め込まれる
        // sの型は、&strになり、文字列スライスになる
        // これは、sがバイナリの特定の位置を表しているということ
        let s: &str = "aaaaaaa";
    }

    // 一番上のスコープの関数fn first_wordのリファクタ
    {
        // &strを受け取るようにすることで、渡す方もStringや文字列どっちでもいける！
        fn first_word(word: &str) -> &str {
            let _byte_word = word.as_bytes();
            for (i, &letter) in  _byte_word.iter().enumerate() {
                if letter == b' ' {
                    return &word[..i];
                }
            }
            &word[..]
        }

        let word = String::from("tomoya kinsho");
        first_word(&word[..]);
        first_word(&word[..1]);
        first_word("hoge");

    }

    {

        let s = 5;
        println!("{:p}", &s);

    }

    {
        let array: [i32; 3] = [1, 2, 3];
        let a: &[i32] = &array[..];
    }

}
