use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("red"), 20);
    }

    {
        // ベクタからハッシュマップを生成できる
        let keys = vec![String::from("blue"), String::from("red")];
        let values = vec![10, 20];

        //  この場合は、型を明示的に指定しないといけない
        // iter()でvecをタプルに変換している
        // zip()でタプルをタプルのベクタを作成している
        let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    }

    // 所有権問題
    {
        let key = String::from("key");
        let value = String::from("value");
        let mut hash_map = HashMap::new();
        hash_map.insert(key, value);
        // println!("{}", key);
    }
    {

        let name_key = String::from("name");
        let name_value = String::from("tomoya");


        let mut name_mapping = HashMap::new();
        name_mapping.insert(name_key, name_value);

        // name_keyはmoveしているので利用できない
        // println!("{}", name_key);


        let name: Option<&String> = name_mapping.get("name");
        // println!("{}", name);


        for (key, value) in &name_mapping {
            println!("{}, {}", key, value);
        }

    }


    {

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // entryはEntry Enumを返す
        // Entry Enumのメソッドである、or_isertを利用することで
        // 存在していれば何もしないような処理ができる
        scores.entry(String::from("yellow")).or_insert(10);
        scores.entry(String::from("Blue")).or_insert(50);
    }

    {
        let sentence: &str = "world world hello";
        let mut map = HashMap::new();

        for word in sentence.split_whitespace() {
            // or_insertはmapの要素の可変参照を返す
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    }

    {

        let sentence = "i am a hero";

        let mut map = HashMap::new();

        for word in sentence.split_whitespace() {
            // or_insertメソッドは、対象keyに紐づくvalueの可変参照（&mut V）を返す
            // なので、sentenceの場合は、iの時、値0への可変参照を返す
            let count = map.entry(word).or_insert(0);
            // その値に1を足したいので、まず参照には加算できないので参照外しをする
            // そして1を加算する
            *count += 1;
        }

        println!("{:?}", map);
    }

    // 練習問題
    {
        let mut list: Vec<i32> = vec![5, 8, 4, 3, 2, 1, 1, 7];

        fn return_mean(list: &Vec<i32>) -> i32 {
            // len()はusizeを返す
            // asでcastできる
            let len = list.len() as i32;
            let mut sum = 0;
            for li in list {
                sum += li;
            }
            sum / len
        }

        fn return_median(list: &mut Vec<i32>) -> i32 {
            // sortの引数は、&mut selfになっている
            list.sort();
            let len = list.len() as i32;
            let median_index = (len / 2) as usize;
            list[median_index]
        }

        fn return_frequency_number(list: &mut Vec<i32>) -> i32 {
            let mut map = HashMap::new();
            for li in list {
                let count = map.entry(li).or_insert(0);
                *count += 1;
            }

            let mut max_value = 0;
            let mut max_key = 0;
            for (key, value) in map {
                // HashMapの型は、HashMap<&i32, i32>になっている
                // keyとして、ヒープに格納されているバケットのポインターを保持している
                // println!("{:p}", key);
                if max_value < value {
                    max_value = value;
                    max_key = *key;
                }
            }
            max_key
        }


        println!("mean: {}", return_mean(&list));
        println!("{:?}", list);

        println!("median: {:?}", return_median(&mut list));

        println!("frequency: {:?}", return_frequency_number(&mut list));


        // HashMapのデータ構造をもっと詳しく調べてみる
        {
            let mut mapp: HashMap<&String, &String> = HashMap::with_capacity(1);
            let key = String::from("hello");
            let value = String::from("hello");
            println!("{:p}, {:p}", &key, &value);
            mapp.insert(&key, &value);
            for (keyy, valuee) in mapp {
                println!("{:p}, {:p}", keyy, valuee);
            }
        }


        // {
        //
        //     enum Group {
        //         Engineer,
        //         Sales
        //     }
        //
        //     let mut list: HashMap<&str, &Vec<&str>> = HashMap::new();
        //
        //     fn add_member(list: &mut HashMap<&str, &Vec<&str>>, user: (Group, &str)) -> () {
        //         match user.0  {
        //             Group::Engineer => {
        //                 match list.get("engineer") {
        //                     Some(li) => {
        //                         println!("{:?}", li);
        //                         ()
        //                     },
        //                     None => {
        //                         // TODO: ここでvecを作ってlistに参照を格納しても
        //                         // スコープを出るとvecはドロップされる
        //                         // ので、コンパイルエラーが出てしまっている
        //                         let mut vec: Vec<&str> = Vec::new();
        //                         list.insert("engineer", &vec);
        //                         ()
        //                     }
        //                 }
        //             }
        //             Group::Sales => {
        //                 match list.get("sales") {
        //                     Some(_) => (),
        //                     None => ()
        //                 }
        //             }
        //         }
        //     }
        //
        //     add_member(&mut list, (Group::Engineer, "kinsho"))
        //
        //
        // }



    }



}
