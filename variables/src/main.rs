fn main() {
    // mutをつけることで、mutableな変数にできる
    // 大きなデータ構造を扱うときになど、毎回新規オブジェクトを作成（コピー）するのは
    // 遅くなるので、そういうときにmutableが有効
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定数は、constで定義
    // 大文字のスネークケースで定義する
    // 型は明示的に指定しないといけない
    const MAX_POINT: u32 = 10000000;


    // シャドーウィング
    // 同じ変数名を使いまわせる

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    // 型が違うくても同じ変数名を使いまわせる
    let spaces = "   ";
    let spaces = spaces.len();

    // 例えば、mutableな変数を使いまわそうとすると
    // 型が違うので、コンパイルエラーになる
    // let mut mut_space = "";
    // mut_space = mut_space.len();


    // 複数の型が推論できる場合は、明示的に型を指定する必要がある


    // ## 整数型
    // 符号付き・符号なしの二通りある。
    // i・・・符号あり（2の補数表現で保持）
    // u・・・符号なし（絶対正の整数になる）

    // 符号ありは、-(2^n-1) ~ (2^n-1 - 1) までを表現できる
    // つまり、8bitの場合、-128 ~ 127
    // なので、以下二つは範囲外とエラーでる
    // let bit_Num: i8  = -129;
    // let bit_num_2: i8 = 128;


    // 符号なしは、0 ~ (2^n-1)まで表現できる
    let result: u32 = "42".parse().expect("");
    // ↓これはコンパイルエラー
    // let types: u32 = -22;


    // 整数型は基準方は、i32型になっている
    // 一番高速らしい

    // ## 浮動小数点型
    // f64, f32がある
    // 基準は、f64で、同じくらい高速で精度高い

    let num: f64 = 2.0;



    // 論理値型
    // true, falseのみ


    let t: bool = true;
    let f: bool = false;


    // char型
    // シングルクォートで表す
    let c = 'z';


    // タプル
    // Scalaと同じような
    let tuple: (i32, char, f64) = (-127, 'a', 4.0);
    let (x, y, z) = tuple;


    let tuple2: (i32, char, f64) = (-127, 'a', 4.0);

    let first = tuple2.0;

    // 配列
    // 固定長になっている
    // 同じ型しか格納できない
    // 後述するベクタ型はサイズを変更できるので便利
    //  配列は、スタック的な使い方なら向いている
    let array = [1, 2, 3];
    let array_first: i32 = array[0];

    // これは、実行時エラーになる
    // 他の低レベル言語だと、存在しないメモリ空間にアクセスできてしまう。このようなチェックをしないから
    // Rustでは、メモリアクセスをいったん許可し、その後即座にプログラム中止させることで守っている。
    // これがメモリセーフと謳っている由縁。
    // ↓エラー内容
    // this operation will panic at runtime
    let index = 10;
    array[index];


}


