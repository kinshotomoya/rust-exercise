use std::task::Context;

fn main() {
    // iteratorは基本的にlazyなので、生成したタイミングでは何も実行されない
    // spark的なことやね
    {
        let v1 = vec![1, 2, 3];
        // 不変参照のiteratorを生成するならiter
        let mut v1_iter = v1.iter();

        // nextメソッドは、iteratorを消費する
        println!("{}", v1_iter.next().unwrap());
        println!("{}", v1_iter.next().unwrap());
        println!("{}", v1_iter.next().unwrap());
        println!("{}", v1_iter.len());

        println!("{}", v1.len());
    }


    {

        let v1 = vec![1, 2, 3];
        // v1の所有権を奪いたいなら、into_iterを使う
        let mut v1_iter = v1.into_iter();

        // nextメソッドは、iteratorを消費する
        println!("{}", v1_iter.next().unwrap());
        println!("{}", v1_iter.next().unwrap());
        println!("{}", v1_iter.next().unwrap());
        println!("{}", v1_iter.len());

        // これはpanicなる
        // println!("{}", v1.len());

    }

    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // sumメソッドを覗くと、所有権を奪うようになっているので、
        // sum()のあとはv1_iter変数を使えない
        let sum: i32 = v1_iter.sum();

        println!("{}", sum);

    }

    {
        let v1 = vec![1, 2, 3];
        // これだでは何も起こらない
        v1.iter().map(|x| x * 2);
    }

    {
        let v1 = vec![1, 2, 3];
        // collect()メソッドでvecを作成する
        // collectメソッドは明示的に型を指定しないといけないので、他のメソッドとchainさせたいときは、
        // ↓のように記載する
        v1.iter().map(|x| x * 2).collect::<Vec<i32>>();

    }

    {

        struct Shoe {
            size: u32,
            style: String
        }

        fn filter_my_shoes(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
            let my_shoes: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == size).collect();
            // println!("{}", shoes.len());

            my_shoes
        }


    }


    {

        struct Counter {
            count: u32
        }

        impl Counter {
            fn new() -> Counter {
                Counter {
                    count: 0
                }
            }
        }

        // カスタムオブジェクトをiteratorにしたかったらIterator traitを実装すればいける
        impl Iterator for Counter {
            // iteratorの中身の型
            type Item = u32;

            // nextメソッドは独自で実装する必要がある
            // 5までをカウントするメソッド
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count > 5 {
                    None
                } else {
                 Some(self.count)
                }
            }
        }


        let counter = Counter::new();

        let res: Vec<u32> = counter.filter(|c| {
            c % 2 == 0
        }).collect();

        assert_eq!(res, vec![2, 4]);

    }


    {
        struct Counter {
            count: u32
        }

        impl Counter {
            fn new() -> Counter {
                Counter {
                    count: 0
                }
            }
        }

        impl Iterator for Counter {
            // iteratorの中身の型
            type Item = u32;

            // nextメソッドは独自で実装する必要がある
            // 5までをカウントするメソッド
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count > 5 {
                    None
                } else {
                    Some(self.count)
                }
            }
        }


        // zipは複数のiteratorの同じindexの要素同士をまとめる
        let res: u32 = Counter::new().zip(Counter::new()).map(|(a, b)| {
            a * b
        }).sum();

        assert_eq!(res, 55);

    }

}
