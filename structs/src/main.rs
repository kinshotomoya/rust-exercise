fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("other@example.com");  // インスタンスがmutableならフィールド値を変更できる
    println!("{}", user1.email);


    fn bind_user(username: String, email: String) -> User {
        User {
            username: username,
            email: email,
            sign_in_count: 1,
            active: true,
        }
    }


    fn bind_user2(username: String, email: String) -> User {
        User { // 仮引数名とフィールド名が同じなら、省略して記載できる
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }


    // emailとusername以外はuser1と同じ値を利用する
    let user2 = User {
        email: String::from("aa"),
        username: String::from("ss"),
        ..user1
    };



    {
        // タプル構造体
        struct Color(u32, u32, u32);
        struct Origin(u32, u32, u32);

        let black = Color(0, 0, 0);
        let origin = Origin(0, 0, 0);
    }



    {

        // struct構造にスライスなどの参照をフィールドに持たせるには、ライフタイムという概念が必要！
        // 基本的に、全てのフィールド値は構造体が無効になるまで有効であり続けて欲しい
        // ライフタイムを指定せずに、参照を持たせようとするとコンパイルに怒られる

        // なので、ライフタイムの説明をするまで所有権があるStringを使う！

        // struct Account {
        //     name: &str,
        //     address: &str
        // }


    }



}

