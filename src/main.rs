#![no_main]
#![no_std]
#![feature(panic_info_message)]

use core::{arch::global_asm, ptr};

mod lang_items;
mod sbi;
mod console;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello World!");
    panic!("Shut down!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        for addr in (sbss as usize)..(ebss as usize) {
            /*逐一字节清零 */
            ptr::write_volatile(addr as *mut u8, 0);
        }
    }
}
