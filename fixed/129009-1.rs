

#![feature(never_type)]

fn main() {}

fn panique(ptr: *const Result<i32, !>) {
    unsafe {
        let Ok(_x) = *ptr;
    }
}
