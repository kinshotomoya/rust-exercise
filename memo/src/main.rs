use std::collections::{HashMap, HashSet, LinkedList};


fn main() {


    // let mut vec = Vec::new();

    let first = &[1.0, 2.0];
    let second = &[3.0, 4.0];

    // extend(&mut vec, first);
    // extend(&mut vec, second);

    // vectorはバッファ足りなくなるとより大きなメモリを確保するために元のバッファを解放し、別アドレスに移動することがある
    // なので↓のようにvectorの参照を引数に渡すと、拡張された場合にはすでに解放されたバッファを差し続けるので
    // コンパイルエラーになってしまう
    // rustでは、このようにデータ競合が起きる可能性をコンパイルで教えてくれる
    // なのでマルチスレッドでのプログラムが簡単にかける3
    // extend(&mut vec, &vec);

    // println!("{:?}", vec);


    // hashMapとHashSet, LinkedListの使い方
    {
        let mut hash_set = HashSet::new();
        let mut hash_map = HashMap::new();
        // scalaのListとは違って双方向Listなので、前後どちらでもtime complexityはO(1)
        let mut list = LinkedList::new();
        let vec = vec!["a", "a", "b", "c", "d", "a", "c"];

        for v in vec {
            if hash_set.insert(v) {
                list.push_back(v);
                hash_map.insert(v, 1);
            } else {
                let count = hash_map.get(v).unwrap_or(&1);
                hash_map.insert(v, *count + 1);
            }
        }
        println!("{:?}", list);
        println!("{:?}", hash_map);

    }


}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for s in slice {
        vec.push(*s)
    }
}

