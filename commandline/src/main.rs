use std::env;
use std::str::FromStr;

fn main() {


    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error"));
    }

    for n in &numbers {
        println!("{}", *n);
    }

    numbers;


}
