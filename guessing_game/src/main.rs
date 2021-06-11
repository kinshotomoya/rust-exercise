use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("数当ててみて！");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        // &で参照を表している
        // 上記で作成したguessオブジェクトのメモリを使い回すことで、毎回新規でオブジェクトを作成する必要がなくなる
        io::stdin().read_line(&mut guess).expect("fail to read ");

        // rustでは、同じ変数名で新しく変数を作成できる（シャドーウィングという）
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ちゃんと数値を入力しろ！！！！💢");
                continue;
            }
        };

        println!("あなたはが予想したのは、{}", guess);

        match guess.cmp(&secret_number) {
            // OrderingはEnumになっていて、サブとして以下三つ存在する
            Ordering::Less => println!("小さいね！"),
            Ordering::Greater => println!("大きいね！"),
            Ordering::Equal => {
                println!("正解！！");
                break;
            },
        };
    }

}
