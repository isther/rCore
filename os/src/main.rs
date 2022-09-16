#![feature(panic_info_message)]
#![no_main]
#![no_std]

use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logger;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }

    clear_bss();

    logger::init();
    error!("Hello, rCore!");

    info!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    trace!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    warn!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
    error!(
        ".stack [{:#x}, {:#x}]",
        boot_stack as usize, boot_stack_top as usize
    );
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
