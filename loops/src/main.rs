fn main() {

    // loop
    // loop {
    //     println!("Hello, world!");
    //     break;
    // }
    //
    //
    //
    // // while
    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{}", number);
    //     number -= 1;
    // }
    //
    // // for
    //
    // let array = [1, 2, 3, 4];
    //
    // // forを使うときは、array -> iteratorにする必要がある
    // for ele in array.iter() {
    //     println!("{}", ele);
    // }
    //
    // for num in (1..4).rev() {
    //     println!("{}", num);
    // }




    // フィボナッチの任意番目を生成する関数
    // 0 1 1 2 3 5 8 13 21 34
    fn create_fibo(x: u64) -> u64 {
        fn aaa(pre_pre: u64, pre: u64, count: u64) -> u64 {
            if count == 0 {
                pre_pre + pre
            } else {
                aaa(pre, pre_pre + pre, count - 1)
            }
        }

        if x == 1 {
            0
        } else if x == 2 {
            1
        } else {
            aaa(0, 1, x - 3)
        }
    }

    let result = create_fibo(50);
    println!("{}", result);

}
