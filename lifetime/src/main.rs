use std::fmt;
use std::fmt::Formatter;

// ライフタイム機能に関して実装をもとに理解していく
// 重要：全ての参照にはライフタイムがある！！！！！！！！！！！！！！！！！！！！
fn main() {

    // これはコンパイルエラーになる
    // {
    //     let r;
    //
    //     // このスコープでxはドロップ（破棄）される
    //     // しかしrは、xへの参照をセットしている。。。
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //
    //     // なので↑のスコープを抜けるとx破棄されているということで、
    //     // コンパイルエラーになる
    //     // rはスコープ内にいるので破棄されていない
    //     println!("{}", r);
    // }

    // これをコンパイラはどうやって実現しているのか
    // これはok
    {
        let x = 5;
        let r = &x;
        println!("{}", r);
    }

    // これはコンパイルエラーになる
    // largest関数の戻り型にlifetime parameterをつけろと言われる
    // この関数自体はstr1かstr2のどちらの参照を返すか分からず、またstr1, str2のライフタイムも分からない
    // なので、コンパイラはstr1, str2が有効か（破棄されていないか）分からない
    {

        // fn largest(str1: &str, str2: &str) -> &str {
        //     if str1.len() > str2.len() {
        //         str1
        //     } else {
        //         str2
        //     }
        // }
        //
        // let string1 = String::from("i am");
        // let string2 = "kinsho";
        //
        // let result = largest(string1.as_str(), string2);
        // println!("largest is {}", result);

    }

    // ライフタイムを指定してコンパイルを通してみる
    {
        // ライフタイム 'aを付与した
        // このように定義することで、str1, str2, 戻り値が全て同じ生存期間を持つことが決定された

        // str1、str2, 戻り値、全て同じジェネリックなライフタイムパラメータ（'a）を定義しているが
        // しかしそれぞれライフタイムが実際には違ってくる（str1もstr2も同じスコープから渡されるわけではないので）
        // なので、実際には'aは、小さい方の具体的なライフタイムになっている！
        fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
            if str1.len() > str2.len() {
                str1
            } else {
                str2
            }
        }

        let string1 = String::from("i am");
        let string2 = "kinsho";

        let result = largest(string1.as_str(), string2);
        println!("largest is {}", result);
    }

    // もう少し詳しく見てみる
    {

        fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
            if str1.len() > str2.len() {
                str1
            } else {
                str2
            }
        }

        // string1のスコープは外側のスコープ
        let string1 = String::from("loger string aaaaaaaaa");

        {
            // string2は内側のスコープ
            let string2 = String::from("shot string");
            // ここで実際に具体的なstring1, string2をlargest関数に渡している
            // この時渡されるライフタイムは、短い方、つまりstring2のライフタイムが渡されるので
            // largest関数の戻り値のライフタイムもstring2と同じになる（実際に戻り値として帰ってくるのは、string1の参照になるが）
            // なので、resultが使えるのはこの内側のスコープの中だけ
            let result = largest(string1.as_str(), string2.as_str());
            // ↑を踏まえるとこれはOK
            println!("longest is {}", result);
        }
    }

    // しかし、↓はコンパイルエラーになる
    // {
    //     fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    //         if str1.len() > str2.len() {
    //             str1
    //         } else {
    //             str2
    //         }
    //     }
    //
    //     // string1のスコープは外側のスコープ
    //     let string1 = String::from("loger string aaaaaaaaa");
    //     let result;
    //     {
    //         // string2は内側のスコープ
    //         let string2 = String::from("shot string");
    //         // ここで実際に具体的なstring1, string2をlargest関数に渡している
    //         // この時渡されるライフタイムは、短い方、つまりstring2のライフタイムが渡されるので
    //         // largest関数の戻り値のライフタイムもstring2と同じになる
    //         result = largest(string1.as_str(), string2.as_str());
    //     }
    //     // ここでresultを参照しているが、resultのライフタイムは引数の短い方string2のスコープと同じなので
    //     // 内側のスコープを抜けた時点でドロップされているので利用できない
    //     println!("longest is {}", result);
    // }

    // 試しにこんなことをやってみた
    // {
    //     // ライフタイムを複数定義して、戻り値のライフタイムをstr1のものにした
    //     // しかし、elseでstr2を返す可能性があるためこれではコンパイルエラーになる
    //     fn largest<'a, 'b>(str1: &'b str, str2: &'a str) -> &'b str {
    //         if str1.len() > str2.len() {
    //             str1
    //         } else {
    //             str2
    //         }
    //     }
    //
    //     // string1のスコープは外側のスコープ
    //     let string1 = String::from("loger string aaaaaaaaa");
    //     let result;
    //     {
    //         // string2は内側のスコープ
    //         let string2 = String::from("shot string");
    //         // ここで実際に具体的なstring1, string2をlargest関数に渡している
    //         // この時渡されるライフタイムは、短い方、つまりstring2のライフタイムが渡されるので
    //         // largest関数の戻り値のライフタイムもstring2と同じになる
    //         result = largest(string1.as_str(), string2.as_str());
    //     }
    //     // ここでresultを参照しているが、resultのライフタイムは引数の短い方string2のスコープと同じなので
    //     // 内側のスコープを抜けた時点でドロップされているので利用できない
    //     println!("longest is {}", result);
    // }

    {
        // これはコンパイル通る
        fn largest<'a>(str1: &'a str, str2: &str) -> &'a str {
            str1
        }
    }

    // {
    //     fn largest<'a>(str1: &str, str2: &str) -> &'a str {
    //         let result = String::from("dvfvfvfvfvvf");
    //         // これはエラー
    //         // resultはこの関数を抜けるとドロップされるので、関数の戻り値型になれない
    //         result.as_str()
    //     }
    //
    //
    // }

    // 構造体定義にライフタイムを注釈をつける
    {

        // ↓ではコンパイルエラー
        // 構造体のフィールドに参照を持たすことができるがその際には、ライフタイム注釈をつける必要がある
        // field partの参照先がImportantExcerptインスタンスよりも短かったら、ImportantExcerptインスタンスは生きているのに、フィールドのpartは
        // は死んでいるみたいな現象が起きてしまうから

        // struct ImportantExcerpt {
        //     part: &str
        // }

        // ImportantExcerptインスタンスのライフタイムが、フィールドpartに保持している参照よりもライフタイムが短いですよと宣言する必要がある
        struct ImportantExcerpt<'a> {
            part: &'a str
        }


        let novel = String::from("oh my god. i am");
        let first_sentance: &str = novel.split(".").next().expect("could not found .");

        ImportantExcerpt {
            part: first_sentance
        };

    }

    // ライフタイム注釈を省略できる
    // ライフタイムの省略規則があり、それに則っていればライフタイム注釈を省略することができる
    {
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }

    // 省略規則は３つ存在する
    // この三つの規則に当てはまらず、ライフタイム注釈をつけていない場合にコンパイルエラーになる！！！

    // 一つ目
    // 引数それぞれ独自のライフタイム引数を持つ
    {
        // fn foo<'a, 'b>(arg1: &'a str, arg2: &'b str)
    }

    // 二つ目
    // 引数一つなら、そのライフタイムが出力ライフタイムにも適応される
    {

        // fn foo<'a>(arg1: &'a str) -> &'a str {
        //
        // }

        // ↓

        // fn foo(arg1: &str) -> &str {
        //
        // }
    }

    // 三つ目
    // 複数の引数が存在するが、メソッド（構造体に定義する関数）なので、&selfがある
    // その場合は、selfのライフタイムが全てに適応される
    // ↓ライフタイム注釈をつけないと、エラーでる
    {

        struct Person<'a>{
            name: &'a str
        }

        impl<'a> Person<'a> {
            // name1を返す可能性があるので、この場合は明示的にライフタイムを指定する
            // このメソッドでコンパイラがやっていることは以下
            // 規則1により、全ての引数に独自のライフタイム注釈がつく
            // 規則2は引数一つの場合なので適応されない
            // 規則３により、戻り値型のライフタイムは&selfと同じになる
            // fn compareNameLenght(&'a self, name1: &'b str) -> &'a str {
            // この時、aとbの関係が分からない。bはaよりもライフタイムが短い可能性もある
            // なので、 どちらにも'aパラメータを付与して、実際には小さい方のライフタイムだと明示的に示す
            fn compareNameLenght(&'a self, name1: &'a str) -> &'a str {
                if name1.len() > self.name.len() {
                    name1
                } else {
                    self.name
                }
            }
        }

        let person: Person = Person{
            name: "name"
        };
        let name = "aaa";

        person.compareNameLenght(name);
    }

    // これはいける
    // メソッドでのライフタイムは関数とは少し解釈が異なる（関数の「短い方のライフタイムが適応される」という観点が）
    {
        struct Person<'a>{
            name: &'a str
        }


        impl<'a> Person<'a> {
            fn compareNameLenght(&'a self, name1: &'a str) -> &'a str {
                if name1.len() > self.name.len() {
                    name1
                } else {
                    self.name
                }
            }
        }

        let result;
        let person: Person = Person{
            name: "name"
        };

        {
            let name: &'static str = "aaaaaaaa";
            // このresultにセットされる参照は、↑nameへの参照になる
            // nameは内側のスコープを抜けると破棄されると思いきや
            // compareNameLenghtメソッドで戻り値型のライフタイムは、personインスタンスと同じだと定義しているので
            // 内側スコープを抜けても破棄されない！！
            println!("name: {:p}", name);
            result = person.compareNameLenght(name);
            println!("result: {:p}", result);
        }
        // ただnameを利用しようとすると、スコープ外だと怒られるので利用はできない
        // println!("{}", name);
        // なので、ここでresultを利用できる（実際にはnameへの参照）
        println!("longer is {}", result);
    }


    // ↑は文字列スライスは、&'staticだから？？
    // 検証してみた
    {
        struct Person<'a>{
            name: &'a str
        }

        struct Person2<'a> {
            name: &'a str
        }

       impl fmt::Display for Person2<'_> {
           fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
               todo!()
           }
       }


        impl<'a> Person<'a> {
            fn compareNameLenght(&'a self, person2: &'a Person2) -> &'a Person2{
                if person2.name.len() > self.name.len() {
                    person2
                } else {
                    person2
                }
            }
        }

        let result;
        let person: Person = Person{
            name: "person"
        };

        {
            let person2 = Person2{
                name: "person2"
            };
            result = person.compareNameLenght(&person2);
            println!("result: {:p}", result);
        }
        // やっぱりそうだった！！
        // ↓はエラーでる
        // resultのライフタイムはperson2のライフタイムになっているので、
        // 内側スコープを出た時点でperson2は破棄されるので、resultは使えない！！！
        // なので、一個前の例では&'staticなので、スコープ外でも利用できていた事になる！！！
        // println!("longer is {}", result);
    }

    // これはエラー
    // {
    //     struct Person<'a>{
    //         name: &'a str
    //     }
    //
    //     impl<'a> Person<'a> {
    //         fn compareNameLenght(&'a self, name1: &'a str) -> &'a str {
    //             if name1.len() > self.name.len() {
    //                 name1
    //             } else {
    //                 self.name
    //             }
    //         }
    //     }
    //
    //     let result;
    //     {
    //         let person: Person = Person{
    //             name: "name"
    //         };
    //         let name = "aaaaaaaa";
    //         // resultにセットされるのは、nameへの参照だが、
    //         // このライフタイムは、personインスタンスと同じライフタイムになる
    //         // personインスタンスのライフタイムは、内側スコープであるので、
    //         // このスコープを抜けると破棄される
    //         result = person.compareNameLenght(name);
    //     }
    //
    //     // なので、ここではresultは利用できない
    //     // すでにpersonインスタンスのライフタイム外になっているので
    //     println!("longer is {}", result);
    //
    //
    // }

    // 静的ライフタイム
    {

        // 文字列は全て静的ライフタイムになっている
        // 直接バイナリに格納されるので、プログラム全体の期間を表している

        let text: &'static str = "text";
        // 省略されて、&strになっているだけ

        let text2: &str = "text2";


        // なので、staticな変数を定義してそれを

    }

    use std::fmt::Display;
    {

        // TはDisplay Traitを実装している型ならannに渡すことができると定義している！
        // 引数は二つ以上あるので各引数にライフタイムパラメータ注釈が必要
        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
            println!("{}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

    }

}
