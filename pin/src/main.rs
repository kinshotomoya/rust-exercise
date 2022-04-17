// fn main() {
//     let self_ref = SelfRef::new(1023);
//     let x = &self_ref.x as *const i32;
//     let ptr_x = self_ref.ptr_x;
//     // ↓実際に変わっている
//     // ptr_xはpre_xのアドレスを指し続けている
//
//     // アドレスを変えないようにするにはどうするのか？
//     println!("now_x: {:?}", x);
//     println!("now_ptr_x: {:?}", ptr_x);
// }


// 自己参照型
struct SelfRef {
    x: i32,
    ptr_x: *const i32
}

impl SelfRef {
    pub fn new(x: i32) -> SelfRef {
        let mut this = Self {
            x,
            ptr_x: std::ptr::null()
        };

        this.ptr_x = &this.x;
        // この時点では同じアドレスポインタを指している
        assert_eq!(&this.x as *const i32, this.ptr_x);
        println!("pre_x: {:?}", &this.x as *const i32);
        println!("pre_ptr_x: {:?}", this.ptr_x);
        // ここでmoveした時点で、xはプリミティブなのでスタック上でコピーされる
        // なのでxのアドレスは変わってしまう
        this
    }
}


// Boxを利用して、そもそもヒープにオブジェクトを作成するようにする
// Boxを利用すると同じメモリアドレスになることがわかる
// Rustのオブジェクト struct enumなどは基本的にスタックに配置される
// BoxをRCはヒープにオブジェクトを作り、そのポインターを返している
// ↑String, Vecなども
// fn main() {
//     let self_ref_box = SelfRefBox::new(1023);
//     println!("now_x: {:?}", &self_ref_box.x as *const i32);
//     println!("now_ptr_x: {:?}", &self_ref_box.ptr_x);
//
//     let test = String::from("tomoya");
//     println!("{:p}", &test);
//     let test2 = test;
//     // test2のスタックアドレスを出力する
//     // もちろんtestと違うスタックアドレスを返す
//     println!("{:p}", &test2);
//
// }

struct SelfRefBox {
    x: i32,
    ptr_x: *const i32
}

impl SelfRefBox {
    pub fn new(x: i32) -> Box<Self> {
        // ヒープにオブジェクトを作る
        let mut this = Box::new(SelfRefBox {
            x,
            ptr_x: std::ptr::null()
        });
        this.ptr_x = &this.x as *const i32;
        println!("pre_x: {:?}", &this.x as *const i32);
        println!("pre_ptr_x: {:?}", this.ptr_x);
        // ヒープへのポインターを返している
        // ヒープ上のメモリは変わらない
        this
    }
}

// Box化以外の方法でオブジェクトをムーブできないようにする
// Pinを使う
fn main() {
    unsafe {
        let mut s1 = SelfRefPin::new(1023);
        let mut s2 = SelfRefPin::new(1109);

        let mut s1_pin = std::pin::Pin::new_unchecked(s1);
        let mut s2_pin = std::pin::Pin::new_unchecked(s2);

        // ↓は二つのメモリアドレスを入れ替えることをするのだが、↑でpinしているので
        // 入れ替えることができなくなっている（コンパイルエラー）
        // 実際には、&mut *s1_pinのように可変参照を取得することができなくなっている
        std::mem::swap(&mut *s1_pin, &mut *s2_pin);
    }

}

#[derive(Debug)]
struct SelfRefPin {
    x: i32,
    ptr_x: *const i32,
    // これはこのオブジェクトはUnpinを実装してないと宣言している。Unpinは基本的にどの型もデフォルトで実装されている。
    // UnpinはムーブしてもOkということを宣言しているもの
    // これがないとムーブされてしまう
    _pin: std::marker::PhantomPinned
}

impl SelfRefPin {
    pub fn new(x: i32) -> Box<Self> {
        let mut this = Box::new(
            Self {
                x,
                ptr_x: std::ptr::null(),
                _pin: std::marker::PhantomPinned
            }
        );
        this.ptr_x = &this.x as *const i32;
        println!("pre_x: {:?}", &this.x as *const i32);
        println!("pre_ptr_x: {:?}", this.ptr_x);
        this
    }
}


