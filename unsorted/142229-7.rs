#![feature(super_let)]

fn main() {
    let _ = const {
        super let x = 1;
        &raw const x
    };
}
