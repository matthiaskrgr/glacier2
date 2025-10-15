//@compile-flags: --edition=2021 -Znext-solver=globally
use core::ptr;
fn foo() -> impl Fn() {
    {
        fn bar(_: *const impl Sized) {}
        let p = ptr::null();
        if false {
            return unsafe { ptr::read(p) };
        }
        if false {
            return unsafe { ptr::read(p) };
        }
    }
    || {}
}

pub fn main() {}
