
#![feature(unsized_tuple_coercion)]
fn main() {
    let x : &(i32, i32, [i32]) = &(0, 1, [2, 3]);
}
