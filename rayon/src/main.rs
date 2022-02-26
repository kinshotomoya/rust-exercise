use rayon::prelude::*;
use std::thread;
// rayonを使ってiterator処理を並列かしていく
fn main() {

    let list = (1..100).collect::<Vec<u64>>();
    list.par_iter().for_each(|i| {
        // println!("current thread name is {}", rayon::current_num_threads());
        // println!("current thread index is {}", rayon::current_thread_index().unwrap());
        // println!("number is: {}", i)
    });

    let map_result: Vec<u64> = list.par_iter().map(|l| {
        println!("current thread index is {}", rayon::current_thread_index().unwrap());
        *l * 2
    }).collect();
    println!("map result: {:?}", map_result);
    // シンプルに計算量が多い処理を並列かするときに有用っぽい



    let mut i: i64 = 1;
    loop {
        // オーバーフローを起こしてしまう。
        // 無限ループをしており、割り当てれるメモリ範囲から溢れ出てしまう
        i *= 2
    }

}
