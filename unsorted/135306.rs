//@compile-flags: -Zlint-mir
#![feature(inline_const_pat)]
struct a(i32);
fn main() {
    match a(5) {
        a(
            const {
                || match 0 {
                    b => 0,
                };
                4
            },
        ) => (),
        c => (),
    }
}
