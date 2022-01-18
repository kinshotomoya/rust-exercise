fn main() {
    // ジェネリクス
    {
        // この関数をi32以外でも利用できるようにしたい
        fn search_largest_num(list: &[i32]) -> i32 {
            let mut largest = list[0];
            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let list1 = vec![100, 20, 4000, 20];
       let aa =  search_largest_num(&list1);
        println!("{}", aa);
    }

    // ジェネリクスを使って↑を抽象化する
    // {
    //     fn largest<T>(list: &[T]) -> T {
    //         let mut largest = list[0];
    //         for &item in list.iter() {
    //             if item > largest { // ここでエラーで、>比較は全ての型で使用できるわけではないので、コンパイルエラーになる
    //                 largest = item;
    //             }
    //         }
    //         largest
    //     }
    //
    //     let i32_list = vec![100, 20, 4000, 20];
    //     let char_list = vec!['a', 'b', 'c', 'd'];
    // }
    {
        // 構造体では
        struct Point<T> {
            lat: T,
            lon: T
        }

        // メソッドでは
        impl<T> Point<T> {
            // 参照のジェネリクスを返す場合は、もちろん&Tを戻り型にする
            pub fn lat(&self) -> &T {
                &self.lat
            }
        }

        Point {
            lat: 1,
            lon: 2
        };
    }

    {

        struct Point<T, U> {
            x: T,
            y: U
        }

        impl <T, U> Point<T, U> {
            fn mixin<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y
                }
            }
        }

    }

    {
        // 単相化を行うことで、ジェネリクスを利用している場合にもパフォーマンスが悪くならないようになっている
        // 単相化・・・コンパイル時に利用している型を当てはめること

        // 例えばOption<T>では

        let one = Some(5); // これはi32
        let two = Some(5.0); // これはf64

        // コンパイル時に具体的なOptionに変換する
        // ↓こんな感じで、
        // なので、実行時には手動でそれぞれの具体的な型を実装したみたいに動くので
        // パフォーマンスに問題ない
        enum Option_i32 {
            Some(i32),
            None
        }

        enum Option_f64 {
            Some(f64),
            None
        }

    }

}
