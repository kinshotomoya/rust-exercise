extern crate core;

use std::ops::Index;

fn main() {
    let num = Number::convert("s");
    Number::from(54);
    println!("{:?}", num);

    let label: Label = Label(String::from("kinsho"));
    // intoするときは明示的に型をしないといけない
    let keyword: Keyword = label.into();
    println!("{:?}", keyword);

}


#[derive(Debug)]
struct Number {
    item: String
}

impl Number {
    // T型をStringに変換する
    // i32 -> Stringは、From<i32> for Stringが実装されていないので、できない
    fn convert<T>(num: T) -> Number
        where T: Into<String>
    {
        Number {
            item: num.into()
        }
    }
}

impl From<i32> for Number {
    fn from(item: i32) -> Number {
        Number {
            item: item.to_string()
        }
    }
}



// custom型 -> custom型もこれでできる
#[derive(Debug)]
struct Keyword(String);

struct Label(String);

impl Into<Keyword> for Label {
    fn into(self) -> Keyword {
        Keyword(self.0)
    }
}
