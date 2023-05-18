// compile-flags: -Ztrait-solver=next
// check-pass

#![feature(trait_upcasting)]

trait Foo: Bar<i32> + Bar<u32> {}

trait Bar<T> {}

fn main() {
    let x: &dyn Foo = todo!();
    let y: &dyn Bar<u32> = x;
    let z: &dyn Bar<u32> = x;
}