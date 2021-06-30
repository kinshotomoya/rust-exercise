mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


fn serve_order() {}

mod back_of_house {

    // enumは要素全て公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // structに関しては、フィールド一個一個にprivate, publicの設定ができる
    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}


// useで関数を持ち込むときは、その親modまで指定してあげるのが慣例的
use crate::front_of_house::hosting;

// 一方でenum, structなどを持ち込むときはフルパスで指定するのが慣例的
use back_of_house::BreakFast;

use std::collections::HashMap;

pub fn eat_at_restaurant() {
    // 絶対パス

    // rustはデフォルトで全て非公開（private）になっている
    // なので、front_of_house親モジュールからhosting子モジュールへの参照はできない
    crate::front_of_house::hosting::add_to_waitlist();


    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::BreakFast::summer("rey");

    meal.toast = String::from("ss");

    // ↓はできない
    // meal.seasonal_fruit = String::from("banana");

    let order1 = back_of_house::Appetizer::Soup;

    hosting::add_to_waitlist();

    BreakFast::summer("sss");
}

use std::io::{Write, self};
// pythonのようにasも使える
use std::fmt as fmtResult;

// 以下のように同じ名前が存在する場合などは、親まで指定してあげる
// fn function1() -> io::Result<()> {
//
// }
//
// fn function2() -> fmt::Result {
//
// }



use std::collections::*;











