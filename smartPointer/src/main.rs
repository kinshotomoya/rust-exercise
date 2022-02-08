fn main() {


    {
        // Box<T>とすることで、ヒープにデータを格納することができる
        // ↓の場合は。
        // Boxはスタックに格納され、ヒープに格納された5を指している
        let b = Box::new(5);
        println!("{}", b);
        // スコープを抜けると、Boxと指しているデータ両方解放される

        // まあこれは、i32に関してはわざわざヒープに持たせなくても良い
        // スタックに持たせとけばいい

    }


    // 実際にBoxが有用なケースを見ていく
    // コンパイラがコンパイル時に知っておく必要があるのは、データのサイズ
    {
        // しかし↓のような再帰型に関しては、コンパイル時にはサイズはわからない
        // ここでBoxを使う

        // enum List {
        //     Cons(i32, List),
        //     Nil
        // }

        // scalaでいうと、Listのデータ構造
        // 再帰になっている
        // let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));

        // これではコンパイル通らない
        // 再帰的なListを定義したので、コンパイラはListに必要な領域をコンパイル時には計算できない


        // コンパイラがどうやって必要な領域を計算しているのか見てみると

        // Messageに必要な領域を計算するために、各列挙子の大きさを計算していく
        // Quitは領域いらん、Moveはi32が2つ。。。など
        // その中からenumは一つしか定義されないので、最大領域を必要とする列挙子の領域が必要だ！とコンパイラは算出する
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Color(i32, i32, i32)
        }

        // しかし、Listの場合は？？
        // まず、Consを調べると、i32とListが必要だとわかる
        // その後、listの療育を調べるために再度Listをみると、Consを調べる必要がでてくる
        // i32とListが必要
        // このように永遠に続いてしまう

    }

    {
        // ↑の時にBox<T>を使う
        // Box<T>はポインタを表しているので、ポインタならコンパイルはコンパイル時に計算できる
        // なぜなら、実際のデータはヒープにあり、ポインタ自身はそのデータのメモリ位置だけを保持しているので。
        // ↓こうすることで、Listのポインタを格納できている

        enum List {
            Cons(i32, Box<List>),
            Nil
        }

        let list = List::Cons(1,
                              Box::new(List::Cons(2,
                                                  Box::new(List::Cons(3,
                                                                      Box::new(List::Nil))
            ))
        ));


    }



}
