
// クロージャ・・・変数に突っ込んだり、関数の引数として渡してあげることのできる匿名関数のこと

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use closure::test;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    test::main2();
    {

    }


}

// TODO: Cacherを以下二点でリファクタする
// 1. 現状どんな値をargに渡されても、1回目に保存した値しか返さないようになっている（valueをhashmapで持たせるようにする）
// 2.　u32しか受け取らないようになっているので、ジェネリクスを使ってu32以外も受け取れるようにする
struct Cacher2<'a, T> where T: Fn(u32) -> u32 {
    calculation: T, // functionを受ける
    value: &'a mut HashMap<u32, u32>
}

impl<'a, T> Cacher2<'a, T> where T: Fn(u32) -> u32{

    fn new(calculation: T, hash_map: &'a mut HashMap<u32, u32>) -> Cacher2<T> {
        Cacher2 {
            calculation,
            value: hash_map
        }
    }

    fn get_or_insert(&mut self, arg: u32) -> &mut u32 {
        // get or insertしたいならEntryを使う
        match self.value.entry(arg) {
            Entry::Occupied(o) => o.into_mut(), // map自身と同じライフタイムを持つvalueを返す
            Entry::Vacant(v) => {
                let res = (self.calculation)(arg);
                v.insert(res) // 格納した値のmutableな参照を返す
            }
        }
        // [メモ]
        // 同じスコープ内では、mutableの借用は一回しかできない
        // immutableの借用が起きていても、その後でmutableの借用はできない！
        // match self.value.get(&arg) {
        //     Some(v) => v,
        //     None => {
        //         &(self.calculation)(arg)
        //         // self.value.insert(v, v); // これはmutableの借用になるのでできない。。。self.value.getでimmutableの借用を行っているから
        //     }
        //
        // }
    }

}


fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculate");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // [メモ]
    // HashMapは、insertとするとその値の所有権はhashmapにムーブされる！！
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut cacher2 = Cacher2::new(expensive_closure, &mut map);

    // これで一回関数が実行されて、valueに値があるならCacherに保存されているvalueが取り出されるだけになった！
    if intensity < 25 {
        println!(
            // 今日は{}回腕立て伏せをしてください！
            "Today, do {} pushups!",
            cacher2.get_or_insert(intensity)
        );

        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            cacher2.get_or_insert(intensity)
        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                cacher2.get_or_insert(intensity)
            );
        }
    }
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculate");
    thread::sleep(Duration::from_secs(2));
    intensity
}


