#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::sleep;

#[no_mangle]
fn main() -> i32 {
    sleep(3000);
    println!("Test sleep OK!");
    0
}
