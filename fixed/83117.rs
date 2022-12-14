#![no_std]
#![feature(lang_items)]

#[lang = "start"] fn start(_: *const u8, _: isize, _: *const *const u8) -> isize {
    0
}

#![no_std]
#![feature(lang_items)]

extern crate ice_test_lib;

use core::panic::PanicInfo;

fn main() {}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] fn eh_personality() {}
