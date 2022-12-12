// src/test/ui/generator/overlap-locals.rs
// run-pass

#![feature(generators)]

fn main() {
    let a = || {
        {
            let w: i32 = 4;
            yield;
            println!("{:?}", w);
        }
        {
            let z: i32 = 7;
            yield;
            println!("{:?}", z);
        }
    };
    assert_eq!(8, std::mem::size_of_val(&a));
}
