mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    // lib.rsはクレートのrootになっているので、
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}


fn serve_order() {}

mod back_to_house {
    fn fix_incorrect_order() {
        // superをつけると、親モジュールからの相対パスになる
        super::serve_order();
    }
}


mod back_of_house {
    pub struct BreakFase {
        pub toast: String,
        seasonal_fruit: String
    }

    impl BreakFase {
        pub fn summer(toast: &str) -> BreakFase {
            BreakFase {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple")
            }
        }
    }
}

pub fn eat_at_bar() {
    let mut breakfast = back_of_house::BreakFase::summer("pan");

    breakfast.toast = String::from("pan2");

    println!("{}", breakfast.toast);

}


mod front_of_house2 {
    pub mod hosting {
        pub fn waitlist() {}
    }
}


use front_of_house2::hosting;

fn eat_at_bar2() {
    hosting::waitlist();
}



// こうすることで、同じファイル名に定義されているモジュールを読み込むようになる
mod family;

use crate::family::parent;

fn execute() {
    parent::calculate();
}


