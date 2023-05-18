// Adapted from https://github.com/sunfishcode/mir2cranelift/blob/master/rust-examples/nocore-hello-world.rs

#![feature(no_core, unboxed_closures, start, lang_items, box_syntax)]
#![no_core]
#![allow(dead_code)]

extern crate mini_core;

use mini_core::*;

#[link(name = "c")]
extern "C" {
    fn puts(s: *const u8);
}

unsafe extern "C" fn my_puts(s: *const u8) {
    puts(s);
}

#[lang = "termination"]
trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        unsafe {
            NUM = 6 * 7 + 1 + (1u8 == 1u8) as u8; // 44
            *NUM_REF as i32
        }
    }
}

trait SomeTrait {
    fn object_safe(&self);
}

impl SomeTrait for &'static str {
    fn object_safe(&self) {
        unsafe {
            puts(*self as *const str as *const u8);
        }
    }
}

#[lang = "start"]
fn object_safe(&self) {
        unsafe {
            puts(*self as *const str as *const u8);
        }
    }

static mut NUM: u8 = 6 * 7;
static NUM_REF: &'static u8 = unsafe { &NUM };

fn main() {
    unsafe {
        let hello: &[u8] = "Hello\0" as &[u8; 6];
        let ptr: *const u8 = hello as *const [u8] as *const u8;
        puts(ptr);

        let world = box "World!\0";
        puts(*world as *const str as *const u8);

        if intrinsics::size_of_val(hello) as u8 != 6 {
            panic(&("", "", 0, 0));
        };

        let chars = &['C', 'h', 'a', 'r', 's'];
        let chars = chars as &[char];
        if intrinsics::size_of_val(chars) as u8 != 4 * 5 {
            panic(&("", "", 0, 0));
        }

        let a: &[u8; 6] = &"abc\0";
        a.object_safe();

        if intrinsics::size_of_val(a) as u8 != 16 {
            panic(&("", "", 0, 0));
        }
    }
}