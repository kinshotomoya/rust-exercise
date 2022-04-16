fn main() {
    trait Animal {
        fn eat(&self);
    }

    struct Lion;
    struct Rabbit;

    impl Animal for Lion {
        fn eat(&self) {
            println!("eat meal")
        }
    }
    impl Animal for Rabbit {
        fn eat(&self) {
            println!("eat carrot")
        }
    }

    // trait objectはコンパイル時にサイズがわからないので、Box化してヒープに格納する必要がある（ヒープへのポインタならサイズはわかるので）
    // Traitオブジェクトを宣言するときは、dynキーワードが必要
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Lion), Box::new(Rabbit)];
}
