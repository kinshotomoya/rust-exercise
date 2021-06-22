fn main() {
    // String型を生成
    // String型だと可変データを格納でき、ヒープに保存される
    let mut s = String::from("hello");
    s.push_str(", world");   // Stringに文字列リテラルを追加する
    println!("{}", s); //　String型には文字列を追加できるので、これはhello, worldと表示される

    // 以下のような文字列リテラルには、文字列リテラルを追加して変更することはできない
    // let mut ss = "hello";
    // ss + ", world";


    // なぜこのようなことが起こるのか

    // 文字列リテラルの場合、コンパイル時に中身が判明しているのでバイトコードに直接ハードコードされる

    // String型は可変・伸長なので、コンパイル時に不明な量のメモリをヒープに確保する
    // String::fromとすることで、OSに対してヒープにメモリを確保するように命令している


    // => しかし、確保したメモリは開放しないといけない
    // java scalaなどのようにGCが存在する言語では、メモリ解放は考えなくても良いのだが、
    // GCが存在しない言語ではメモリ解放を考えないといけない

    // Rustはちょっと違った道を歩んでいる

    {
        let s = String::from("hello"); // ここでは開始

    }  // スコープを抜けると自動的にメモリは解放される

    // ↑のとじカッコで内部的にdrop関数を呼び出しており、メモリを解放している



    // 以下は、xをyにコピーしている
    // これらはスタックに保存される
    let x = 5;
    let y = x;


    // これらはどうだろうか
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); s1の所有権はs2にmoveされているのでコンパイルエラー

    // s1にはメモリへのポインター・長さ・許容量の三つが格納されて、スタックに格納される
    // s1のポインター先には、実際のhelloという文字を保持したヒープ上のメモリが存在している

    // s1をs2にコピーしているということは、上記の一つ目であるメモリへのポインター・長さ・許容量の三点をs2にコピーしているということになる
    // つまり、ヒープ上のメモリはコピーしない
    // ヒープ上の対象のメモリをs1, s2の両方から指しているという状態になる


    // クローン
    // クローンすると、ヒープのデータがそのままコピーされるんで
    //  ↓の例だと、再びs2を問題なう利用できる
    let s3 = s2.clone();
    println!("{}, {}", s2, s3);


    // スタック上のコピー
    // javaでいうプリミティブ型はスタック上のコピーになる
    let a = 5;
    let b = a;
    println!("{}, {}", a, b);


    let ss = String::from("kinsho");
    take_ownership(ss);
    // println!("{}", ss); ssは既に関数にmoveしているのでここでは利用できない


    fn take_ownership(name: String) {
        println!("{}", name);
    }// nameのヒープ上のデータがdropされる（解放される）



    let full_name = String::from("kinsho tomoya");
    let full_name_backed = take_and_give_back(full_name);

    println!("{}", full_name_backed);

    fn take_and_give_back(full_name: String) -> String {
        full_name
    }



}
