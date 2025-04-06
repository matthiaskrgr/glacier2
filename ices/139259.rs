#![feature(generic_const_exprs)]
#![feature(min_generic_const_args)]

struct A {
    arr: usize,
    a:i32
}
struct B<const N: A> {
    arr: [u8; N.arr],
    a:i32
}
struct Struct(u32);
const C: Struct = Struct(42);
fn main() {
    let b = B::<C> {};
    assert_eq!(b.arr.len(), 5);
}
