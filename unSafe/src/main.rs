fn main() {

    // 生ポインタ
    {
        let mut num = 5;
        // 生ポインタを作成している;
        // これらは参照が有効であるとわかる
        let r1 = &num as *const i32;
        let r2 = &mut num as *const i32;
    }

    {
        // 参照が有効なのかわからない
        let address = 0x012345i32;
        let r1 = &address as *const i32;
    }

    // 使い所がよくわからないので必要になったら学ぶ

}
