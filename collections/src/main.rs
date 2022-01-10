fn main() {
    // ベクタ
    {
        let v: Vec<i32> = Vec::new();
        let _v1 = vec![1, 2, 3];
    }
    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        println!("{:?}", v);
    }
    {
        // ベクタもstruct同様、スコープを抜けるとドロップされる
    }
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        let third: Option<&i32> = v.get(2);
    }

    {
        let v = vec![1, 2, 3];
        // ↓これは実行時にパニックになる
        // let not_exists = &v[100];
        // ↓これはNoneが返るのでパニックにならない
        let not_exists = v.get(100);
    }
    {

        // 可変借用と不変借用は両立できない
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

        // ここで不変借用
        let first: &i32 = &v[0];
        let second: &i32 = &v[1];

        //　ここで可変借用
        // v.push(6);

        // これではコンパイルエラー
        println!("The first element is: {}, {}", first, second);

        // なぜこのようなことがコンパイルエラー？？
        // ベクタに要素を新規追加する際に、末尾要素の隣にメモリが空いていない場合
        // 全ての要素をコピーして別のメモリに移動する必要がある（Arrayと同じ）
        // その際に、上記でいうfirst, secondが参照を保持していて、その参照はベクタのメモリが移動すると
        // 全くの意味のないメモリ参照を保持していることになり、バグに繋がるから
        // このような不変借用と可変借用は両立できないようになっている！！

    }

    // for

    {
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 10
        }
        println!("{:?}", v);
    }

    {
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            // iは参照値である（つまりベクタ要素のメモリポインタ）
            // そいつに50を加算することはできない
            // i += 50;

            // そーゆー時には参照外しを使う
            // *をつけることで、参照元までたどり着くことができる
            *i += 50
        }

        println!("{:?}", v);
    }

    // enumで
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }

        let row = vec![
            SpreadsheetCell::Int(1),
            SpreadsheetCell::Float(2.0),
            SpreadsheetCell::Text(String::from("hello"))
        ];

    }
}
