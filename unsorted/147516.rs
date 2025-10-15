use std::mem::size_of;
fn distance_between<T>(dest: *const T, origin: *const T) -> usize {
    match size_of::<T> {
        _ => origin as usize,
    }
}
struct S;
const X: [S; 4] = [S, S, S, S];
const Y: *const S = X.as_ptr();
const Z: *const S = (Y as *const u8).wrapping_add(2) as *const S;
pub fn main() {
    println!("{}", distance_between(Z, Y))
}
