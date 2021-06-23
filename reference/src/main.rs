fn main() {
    // 参照

    let name = String::from("kinsho tomoya");
    take_name(&name); // 参照している。所有権は渡さず、ヒープ上のデータへのポインタをさす、ポイントを作成している
    println!("{}", name);

    let name_length = calcurate_length(&name);
    println!("{}", name_length);

    fn take_name(name: &String) {
        println!("my name is {}", name);
    }

    fn calcurate_length(name: &String) -> usize {
        // nameのことを借用という
        name.len()
    }


    do_something(&name);

    fn do_something(name: &String) {
        // name.push_str("my name is")  借用したデータの中身は変更できない
    }


    // mutにすると、借用先でその値を変更することができる
    let mut full_name = String::from("full name");

    do_something_mutable(&mut full_name);

    println!("{}", full_name);

    fn do_something_mutable(name: &mut String) {
        name.push_str("my name isssssss");
    }

    let mut ss = String::from("ss");
    let ss1 = &mut ss;
    // let ss2 = &mut ss; // 特定のスコープではmutableな参照は一つしか持てない
    println!("{}", ss1);

    let mut sss = String::from("sss");
    let sss1 = &sss;
    let sss2 = &sss;
    // let sss3 = &mut sss; // 既にimmutableで参照が使われている場合は、一つもmutableで参照できない
    println!("{}, {}", sss1, sss2);


    // 参照が宙に浮くことがない
    // ポインタだけ残って、参照先データが消えてしまうことが他の言語では発生する可能性がある
    // rustはコンパイラがそれを防いでくれている

    let nothing = dungle();

    fn dungle() -> &String {
        // 参照を返しているが、この関数を抜けると参照先データ（s）は削除されるので
        // ポインタだけ浮いてしまう
        let s = String::from("dungle");
        &s
    }

}
