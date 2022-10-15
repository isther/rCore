#![no_main]
#![no_std]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;

mod logger;
use log::*;

#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

pub mod batch;
mod sync;
pub mod syscall;
pub mod trap;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();
    info!("Hello, rCore!");

    trap::init();
    batch::init();
    batch::run_next_app();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|p| unsafe { (p as *mut u8).write_volatile(0) });
}
