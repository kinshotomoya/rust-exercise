fn main() {
    {
        let mut s = String::new();

        let s2 = "hello".to_string();

        let s3 = String::from("hello");

    }

    // Stringは、文字列を追加することで伸ばすことができる
    {
        let mut s = String::from("hello");
        s.push_str("world");
        s.push('r');

        println!("{}", s);
    }

    // 文字列結合
    {
        let s1 = String::from("hello ");
        let s2 = String::from("world");

        // +メソッドででは、第一引数にself, 第二引数に&strを受け取る
        // なので、s2は文字列スライスを受け渡す必要がある
        // しかし、&s2みたいな&stringでもいい
        // +メソッドが内部的に強制型変換を行ってくれているから&s2を渡しても問題ない

        // つまりここでは、s1の所有権を奪いs2の中身をコピーして追加し、その結果を返している
        let s3 = s1 + &s2[..];
        println!("{}", s3);
        // println!("{}", s1); value borrowed here after move error
    }

    {
        let s = String::from("hwllo");
        // s[0];  これはできない
        // なぜなら各文字列はUTF-8方式でエンコードされており、その値が各メモリに保存されているので
        // 正しい値をユーザーに返すことができないから、Rustではコンパイルの時点でエラーにしている


        let name = "金正朋也";
        // ↑平仮名・漢字の場合は、一文字3バイトになっている
        // ↓のように文字列スライスを利用する場合は、バイト数でスライスする文字列を指定する
        // なので、漢字の場合は0..3と先頭から3バイトを指定することで、やっと一文字目を取得できる
        let s1 = &name[0..3];
        println!("{}", s1);

        let hiragana = "きんしょうともや";
        let s2 = &hiragana[0..3];
        println!("{}", s2);

    }
}
