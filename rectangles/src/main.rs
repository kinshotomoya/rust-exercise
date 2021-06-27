fn main() {
    {
        let width1 = 30;
        let higth1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, higth1)
        );

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
    }

    // タプルでリファクタ
    {
        let area1 = (30, 50);
        println!(
            "The area of the rectangle is {} square pixels.",
            area(area1)
        );

        fn area(dimentions: (u32, u32)) -> u32 {
            dimentions.0 * dimentions.1
        }

    }

    // structでリファクタ
    {

        struct Rectangle {
            width: u32,
            height: u32
        }

        let rect = Rectangle {
            width: 30,
            height: 50
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect)
        );

        // &rectを関数の引数にしているので、ここでもまだrectの所有権は有効
        println!("{}", rect.height);

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

    }

    {
        // これをつけると、独自構造体にString:fmtの実装が必要なくprintln!に出力できる
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        let rect = Rectangle {
            width: 30,
            height: 50
        };

        //
        println!("{:#?}", rect);
    }

    // areaをインスタンスメソッドにする
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let rect = Rectangle { width: 30, height: 50 };
        println!(
            "The area of the rectangle is {} square pixels.",
            rect.area()
        );

    }

    // 引数を二つ以上受け取るインスタンスメソッドを定義する
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 40, height: 60 };

        println!("{}", rect1.can_hold(&rect2));

    }

    //  関連関数
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        impl Rectangle {
            // selfを受け取らない関数を定義できる
            // これは、インスタンスメソッドではなく関連関数と呼ばれる
            // コンストラクタによく利用される
            fn square(size: u32) -> Rectangle {
                Rectangle { width: size, height: size }
            }
        }

        let rec = Rectangle::square(44);

        println!("{:?}", rec);
    }

}
