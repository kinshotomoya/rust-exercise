fn main() {
    let self_ref = SelfRef::new(1023);
    let x = &self_ref.x as *const i32;
    let ptr_x = self_ref.ptr_x;
    // ↓実際に変わっている
    // ptr_xはpre_xのアドレスを指し続けている
    println!("now_x: {:?}", x);
    println!("now_ptr_x: {:?}", ptr_x);
}


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
