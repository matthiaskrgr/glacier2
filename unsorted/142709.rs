#![feature(generic_const_exprs)]
struct X<
    const FN: fn() = {
        || {
            let _: fn([A; B]) -> B = B::B;
        }
    },  
>;
fn main() {}
