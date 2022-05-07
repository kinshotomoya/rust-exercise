use std::collections::{HashMap, HashSet, LinkedList};
use std::error::Error;
use std::fmt::Formatter;


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

    // カスタムエラー型を定義
    // ↓こんな感じでErrorを実装したりなど手間がかかる
    {
        #[derive(Debug, Clone)]
        struct JsonError {
            pub message: String,
            pub line: usize,
            pub column: usize
        }

        // Displayを実装しないといけない
        impl std::fmt::Display for JsonError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "")
            }
        }

        // カスタムのエラー型が標準エラーと同じように使えるようにするには、
        // std::error::Errorを実装している必要がある

        impl std::error::Error for JsonError{}

        fn return_result<T>() -> Result<T, JsonError> {
            Err(
                JsonError {
                    message: String::from(""),
                    line: 1,
                    column: 2
                }
            )
        }

    }

    // thiserrorクレートを使う
    {
        use thiserror::Error;

        // thiserrorクレートを使うとErrorの実装などの手間が省ける
        // #[derive(Error)]がthiserrorを使う
        #[derive(Error, Debug)]
        #[error("{message}")]
        struct JsonError {
            pub message: String,
            pub line: usize,
            pub column: usize
        }

    }

    {
        enum Animal {
            Person,
            Fish(String),
            Mammalian {id: u32, code: String}
        }

        #[derive(Debug)]
        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>)

        }

        impl<T> BinaryTree<T> where T: Ord {
            fn add(&mut self, value: T) {
                match *self {
                    BinaryTree::Empty => {
                        *self = BinaryTree::NonEmpty(Box::new(
                            TreeNode {
                                element: value,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty
                            }
                        ))
                    }
                    BinaryTree::NonEmpty(ref mut node) => {
                        if value <= node.element  {
                            node.left.add(value)
                        } else {
                            node.right.add(value)
                        }
                    }
                }
            }
        }

        #[derive(Debug)]
        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>
        }

        let brother_node = BinaryTree::NonEmpty(
            Box::new(TreeNode {
                element: "brother",
                left: BinaryTree::Empty,
                right: BinaryTree::Empty
            })
        );

        let sister_node = BinaryTree::NonEmpty(
            Box::new(TreeNode {
                element: "sister",
                left: BinaryTree::Empty,
                right: BinaryTree::Empty
            })
        );

        let mother_node = TreeNode {
            element: "mother",
            left: brother_node,
            right: sister_node
        };

        // NonEmptyの要素をBoxにすることでヒープに割り当てることができるようになり、
        // 問題なくこなせるようになる
        let root_tree = BinaryTree::NonEmpty(
            Box::new(
                mother_node
            )
        );

        let mut tree = BinaryTree::Empty;
        tree.add("s");
        tree.add("wee");

        println!("{:?}", tree);

    }

    {
        #[derive(Debug)]
        struct Account {
            name: String,
            language: String
        }

        let account = Account{name: String::from("tomoya"),language: String::from("japanese")};

        match account {
            Account{ref name, ref language} => {
                println!("{:?}", account);
                println!("{}, {}", name, language);
            }
        }
    }


}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for s in slice {
        vec.push(*s)
    }
}

