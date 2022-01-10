fn main() {


    // 文と式という概念が存在する
    // 文・・・値を返さない
    // 式・・・なんらかの値を返す
    // 関数定義は文になる！！！！

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);
    // なので、↓のように変数に格納しようとしても関数は何も返さないのでエラー出る
    // let result = another_function(32);
    // println!("{}", result);


    let return_result: i32 = return_function(32);
    println!("{}", return_result);

    for number in (1..4).rev() {
        println!("number is {}", number)
    }

}


// ただの関数定義だとこれは文になるので、何も値を返さない
// 式の場合は、セミコロンはつけない
fn another_function(x: i32) {
    x + 1;
}

// 戻り型を指定している
fn return_function(x: i32) -> i32 {
    x * 2
}

// fn return_error() -> i32 {
//     5;
// }
