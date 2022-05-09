#![feature(naked_functions)]
use std::arch::asm;

// usizeは計算機のワード長
// 64bit PCなので2^64になる
const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;
const MAX_THREAD: usize = 4;
static mut RUNTIME: usize = 0;

fn main() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("THREAD 1 STARTING");
        let id = 1;
        for i in 0..10 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 1 FINISHED");
    });
    runtime.spawn(|| {
        println!("THREAD 2 STARTING");
        let id = 2;
        for i in 0..15 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 2 FINISHED");
    });
    runtime.run();

}

pub struct Runtime {
    threads: Vec<Thread>,
    current: usize
}

impl Runtime {
    pub fn new() -> Self {
        let base_thread = Thread {
            id: 0,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running
        };

        let mut threads = vec![base_thread];
        let mut available_threads = (1..=MAX_THREAD).map(|id| {
            Thread::new(id)
        }).collect::<Vec<Thread>>();
        threads.append(&mut available_threads);

        Runtime {
            threads,
            current: 0
        }
    }

    // RUNTIMEの生ポインタをグローバル定数に格納している
    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn run(&mut self) -> ! {
        while self.t_yield() {}
        std::process::exit(0);
    }

    // threadが終了した時に呼ばれるメソッド
    // base threadじゃなければ、stateをavailableに変更する
    fn t_return(&mut self) {
        if self.current != 0 {
            self.threads[self.current].state = State::Available;
            self.t_yield();
        }
    }

    #[inline(never)]
    //スレッドを探してタスクを実行する
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        // State::Readyなthreadを見つかるまでwhileを回し続ける
        while self.threads[pos].state != State::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0
            }
            if pos == self.current {
                return false
            }
        };

        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }

        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        // CPUに命令している
        // new threadを実行するように
        unsafe {
            let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = &self.threads[pos].ctx;
            asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
        }

        self.threads.len() > 0
    }



    pub fn spawn(&mut self, f: fn()) {
        let available = self.threads.iter_mut().find(|s| {
            s.state == State::Available
        }).expect("no available thread now");

        let size = available.stack.len();

        // 対象スレッドのスタックに、実行する関数のポインタを格納している
        // 関数の実行順番 f -> skip -> guard
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, f as u64);
            available.ctx.rsp = s_ptr.offset(-32) as u64;
        }

        available.state = State::Ready;
    }
}


fn guard() {
    unsafe {
        // Runtime自信を取得する
        // init時にRuntimeのポインタをグローバル定数に格納していたので、そこから取得する
        let rt_ptr = RUNTIME as *mut Runtime;
        // thread終了
        (*rt_ptr).t_return();
    }
}

#[naked]
unsafe extern "C" fn skip() {
    // これはスタックから次の値を取り出すアセンブリコード
    asm!("ret", options(noreturn))
}

pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    }
}


// レジスタの値を読み出すアセンブリを書いている
// これは↓で呼ばれている
// asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
#[naked]
#[no_mangle]
unsafe extern "C" fn switch() {
    asm!(
    "mov [rdi + 0x00], rsp",
    "mov [rdi + 0x08], r15",
    "mov [rdi + 0x10], r14",
    "mov [rdi + 0x18], r13",
    "mov [rdi + 0x20], r12",
    "mov [rdi + 0x28], rbx",
    "mov [rdi + 0x30], rbp",
    "mov rsp, [rsi + 0x00]",
    "mov r15, [rsi + 0x08]",
    "mov r14, [rsi + 0x10]",
    "mov r13, [rsi + 0x18]",
    "mov r12, [rsi + 0x20]",
    "mov rbx, [rsi + 0x28]",
    "mov rbp, [rsi + 0x30]",
    "ret", options(noreturn)
    );
}

#[derive(PartialEq)]
enum State {
    Available, // 新しいタスクを割り当てることができる
    Running, // 動いている最中
    Ready // 準備完了
}

#[derive(PartialEq)]
struct Thread {
    id: usize,
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State
}

impl Thread {
    pub fn new(id: usize) -> Self {
        Thread {
            id,
            stack: vec![0_u8; DEFAULT_STACK_SIZE], // 8bitを0で埋めている
            ctx: ThreadContext::default(),
            state: State::Available
        }
    }
}


#[derive(Default, PartialEq)]
#[repr(C)] // C言語の構造体と互換性を持つ構造体を定義するのに必要
// CPUが実行するのに必要なレジスタのデータを保持している
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}
