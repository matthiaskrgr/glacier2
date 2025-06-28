#![feature(generic_const_exprs)]
struct X<
    const FN: () = {
        || {
            let _: [(); B];
        };
    },  
>;
fn main() {}
