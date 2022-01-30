pub mod test {

    pub fn main2() {
        {
            // closureに型情報を付与する必要はないが、明示的に型注釈を付与することもできる
            let function = |num: u32| -> u32 {
                num
            };

            // ↓と一緒
            let function2 = |num: u32| {
                num
            };
        }


        {

            // 全部同じ定義！！
            fn function(num: u32) -> u32 {
                num + 1
            }
            let function2 = |num: u32| -> u32 {
                num + 1
            };

            let function3 = |num: u32| {
                num + 1
            };

            let function4: fn(u32) -> u32 = |num: u32| num + 1;

        }

        // この場合はエラーでる
        {

            let function = |num| num;

            let res1 = function(String::from("kinsho"));
            // let res2 = function(5); // ↑でclosureの引数型はStringであると推測されたので、u32は引数に指定できない
        }

        // Fn traitを
        // 使ってclosureを保持する構造体を作ってみる
        // このCacherでmain.rsのコードをリファクタリングしてみる
        {
            struct Cacher<T> where T: Fn(u32) -> u32{
                calculation: T,
                value: Option<u32>
            }

            impl<T> Cacher<T> where T: Fn(u32) -> u32 {
                // Cacherを作った時点では、関数だけ保持しておいて、関数の実行結果であるvalueにはNoneを格納する
                fn new(calculation: T) -> Cacher<T> {
                    Cacher {
                        calculation,
                        value: None
                    }
                }

                //
                fn value(&mut self, arg: u32) -> u32 {
                    match self.value {
                        Some(v) => v,
                        None => {
                            let v = (self.calculation)(arg);
                            self.value = Some(v);
                            v
                        }
                    }
                }

            }

            let func = |num: u32| num * 2;
            let mut cacher = Cacher::new(func);
            cacher.value(3);

        }

    }
}
