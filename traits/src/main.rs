use std::fmt::Display;

fn main() {

    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}", self.author)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}", self.content)
            }
        }
    }


    {
        pub trait Content {
            fn call() -> String {
                String::from("this is a book.")
            }
            fn summarize(&self) -> String;
        }

        pub  struct Book {
            pub name: String,
            pub author: String
        }


        impl Content for Book {
            fn summarize(&self) -> String {
                format!("{}", self.name)
            }
        }

    }

    {

        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub trait Price {
            fn sum(&self) -> u32;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
            pub price: u32
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}", self.author)
            }
        }

        impl Price for NewsArticle {
            fn sum(&self) -> u32 {
                self.price
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}", self.content)
            }
        }



        // こんな感じでtraitを引数の型に指定することができる
        // その時は、implをつけてあげる必要がある
        pub fn notify(item: &impl Summary) {
            format!("{}", item.summarize());
        }

        // ↑は実は↓の実装の糖衣構文である
        // ジェネリックな型にSummaryを指定している感じ
        pub fn notify2<T: Summary>(item: &T) {
            println!("{}", item.summarize());
        }

        // さらに引数を二つ持つ場合
        pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
            println!()
        }

        // ↑は↓のようにかける
        pub fn notify4<T: Summary>(item1: &T, item2: &T) {
            println!()
        }

        // 複数のtrait境界を指定できる
        // +を指定する
        pub fn notify5(item: &(impl Summary + Price)) {
            item.summarize();
            item.sum();
        }

        // ジェネリックでもできる
        pub fn notify6<T: Summary + Price>(item: &T) {

        }

        // 多い場合は、whereが使える
        pub fn notify7<T>(item: &T)
            where T: Summary + Price
        {
        }

        // 戻り値としてtraitの型を指定する

        pub fn notify8<T>(item: &T) -> impl Summary {
            Tweet {
                username: String::from(""),
                content: String::from("")
            }
        }

    }

    {
        // ジェネリクスを利用したlargest関数を完成させる
        fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
            // Tは、i32などのようなプリミティブな型か、Summaryのようなヒープに保存しないといけない型かコンパイラはわからない
            // なので、list[0]みたいなvectorの一要素だけをムーブするような書き方はできない（そもそもvectorの一要素のみをムーブするのは許可されていない）
            // なので一要素の参照を渡すよう！！
            let mut largest = &list[0];

            for item in list.iter() {
                // 参照外しで、実データ同士を比較する
                if *item > *largest { // >メソッドを利用するためには、Tは>メソッドを実装した型が入ると定義する必要がある。トレイト境界にstd::cmp::PartialOrdを定義してあげる
                    largest = &item;
                }
            }
            largest
        }

        let list = vec![10, 40, 3000, 55544454, 11111];
        let max = largest(&list);
        println!("{}", max);
    }

    {
        // メソッド実装を条件分けする
        // こんな感じで、
        pub struct Pair<T> {
            x: T,
            y: T
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Pair {
                    x,
                    y
                }
            }
        }

        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x > self.y {
                    println!("{}", self.x)
                }
            }
        }


    }

    {

        struct Name {
            pub name: String
        }

        fn test(list: &[i32]) {
            list[0];
        }

        let list = vec![Name{name: String::from("kinsho")}];
        let list2 = &list;
        // let list3 = list2[0]; // これはvectorの0番目の要素をlist3にムーブしようとしている。が、一部要素だけムーブするのは許可されていないので、エラーが出る


    }
    {
        // これは、プリミティブな型ならok
        // なぜなら、ムーブという概念は存在せず、shallow copy（スタックにそのままコピーされる）ので
        let list = vec![1, 2, 3];
        let list2 = &list;
        let list3 = list2[0];
    }

    {

        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}", self.headline)
            }
        }

        pub trait Sample {
            fn execute(&self);
        }

        // Summaryを実装している任意の方に対して、メソッドを生やすことができる
        impl<T: Summary> Sample for T {
            fn execute(&self) {
            }
        }

        let tweet = NewsArticle {
            headline: String::from("newsss!!!")
        };

        tweet.execute()



        // こうすることで、Summary traitを実装する任意の方に対して、Sampleで定義されたexecuteメソッドを利用できる

    }


}
