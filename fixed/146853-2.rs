#![feature(c_variadic)]
trait Trait {
    unsafe extern "C" fn foo(_: ...) -> i32 {
        0
    }
}
impl Trait for i32 {}
fn main() {
    let _f = i32::foo as unsafe extern "C" fn(...) -> i32;
}
