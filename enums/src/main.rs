fn main() {
    {
        enum IpAddrKind {
            V4,
            V6
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        fn route(ip: IpAddrKind) {

        }

    }

    {
        enum IpAddrKind {
            V4,
            V6
        }

        struct IpAdd {
            kind: IpAddrKind,
            ip: String
        }

        let home = IpAdd {
            kind: IpAddrKind::V4,
            ip: String::from("127.0.0.1")
        };

    }

    // enumに直接データを格納することができる
    // 無駄な構造体を定義する必要がない！！！
    {
        #[derive(Debug)]
        enum IpAddKind {
            V4(String),
            V6(String)
        }

        let home = IpAddKind::V4(String::from("127.0.0.1"));

        println!("{:?}", home);

    }

    {
        enum IpAddrKind {
            V4(u8, u8, u8, u8),
            V6(String)
        }

        let home = IpAddrKind::V4(127, 0, 0, 1);

    }

    // enumにもメソッドを定義できる
    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32)
        }

        impl Message {
            fn call(&self) {
                println!("{:?}", self);
            }
        }

        // structと同様に、インスタンスを作成しメソッドを呼び出すことができる
        let m: Message = Message::Write(String::from("sss"));
        m.call();

    }

    // enum optionを見ていく！！！！
    // Rustにはnullは存在しないので、代わりにOptionを使うようになっている
    {
        // 標準ライブラリに以下のようにOptionは定義されている
        enum Option<T> {
            Some(T),
            None
        }
    }


    {
        let some_number = Some(5);
        let some_string = Some(String::from(""));

        // Noneの場合は、明示的に型を指定してあげる必要がある
        // なぜなら、コンパイラがTの値を決めれないから
        let absent_number: Option<u32> = None;
    }

    // match
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cent(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 32,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

    }

    {
        #[derive(Debug)]
        enum UsState {
            NY
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cent(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 32,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) =>{
                    println!("{:?}", state);
                    25
                },
            }
        }


        value_in_cent(Coin::Quarter(UsState::NY));

    }


    // Optionでmatch
    {

        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(i) => Some(i + 1),
                _ => None
            }
        }

        plus_one(None);
        plus_one(Some(5));
    }

    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("tree"),
            _ => ()
        }

        // ↓の糖衣構文
        // しかし、この場合には包括性はない
        if let Some(3) = some_u8_value {
            println!("tree")
        }

        // matchでは冗長すぎる条件なら、if letを使える！
        // elseも利用可能


        enum UsState {
            Ny
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }


        let mut count = 0;
        let coin = Coin::Quarter(UsState::Ny);


        if let Coin::Quarter(state) = coin {
            println!("Quarter!!");
        } else {
            count += 1;
        }

    }

    {

        #[derive(Debug)]
        struct Info {
            name: String,
            address: String,
            phone_number: String,
            active: bool
        }

        impl Info {
            fn apply(name: String, address: String, phone_number: String) -> Info {
                Info {
                    name,
                    address,
                    phone_number,
                    active: true
                }
            }


            fn get_phone_number(&self) -> &String {
                println!("{:?}", self);
                // self.phone_numberを返そうかと思ったが、
                // self.phone_numberと書くと、Infoオブジェクトのphone_numberの所有権のみ
                // 外部に渡すことになるので、cannot move outみたいなコンパイルエラーが出る
                // なので、参照を返すようにしている
                &self.phone_number
            }

            // プリミティブ型は、スタックにコピーされるので
            // 参照ではなくそのままコピーした値を返すことができる
            fn get_active(&self) -> bool {
                self.active
            }

            // ↑ただコピーしたくないので、参照を返すこともできる！
            fn get_active_reference(&self) -> &bool {
                &self.active
            }

        }


        enum GymKind {
            GOLDGYM(Info),
            ANYTIME(Info),
            CONAMI(Info)
        }

        let gym = GymKind::GOLDGYM(Info::apply(String::from(""), String::from(""), String::from("090")));


        if let GymKind::GOLDGYM(info) = gym {
            let gold_phone_number = info.get_phone_number();
            println!("{}", gold_phone_number);

            let status = info.get_active();
            println!("{}", status);

            let status_refe = info.get_active_reference();
            println!("{}", status_refe);
        }


    }

}
