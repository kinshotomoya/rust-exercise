use core::arch::asm;
use std::thread::Thread;

fn main() {
    const SSIZE: isize = 48;

    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];

    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE); // stackの底（最初）のアドレスを取得
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64); // hello関数のポインタをスタックの16番目に上書き
        ctx.rsp = sb_aligned.offset(-16) as u64;

        for i in (0..SSIZE).rev() {
            println!("mem: {}, val: {}",
                     sb_aligned.offset(i as isize) as usize,
                     *sb_aligned.offset(i as isize))
        }


        gt_switch(&mut ctx);
    }

    // 実行すると、heloooooooと標準出力される
    // cpuを直接命令できた
}


#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64
}


fn hello() -> ! {
    println!("helooooooo");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]", // 0x00に格納された値をrspレジスタのメモリ位置{0}（スタックの最上位）に移動させる
        "ret", // スタックの一番上にcpuをジャンプさせる
        in(reg) new,
    )
}
