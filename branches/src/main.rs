fn main() {

    let number = 5;

    // ifに渡す値は、bool型じゃないとあかん
    if number < 5 {
        println!("Hello, world!");
    } else if number < 5 {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }



    // ifは式なので、何かしらの値を返す
    let condition = true;

    let number: i32 = if condition {
        5
    } else {
        4
    };

}
