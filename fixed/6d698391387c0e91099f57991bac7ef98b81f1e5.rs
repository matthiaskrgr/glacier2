#![feature(generic_const_exprs)]
fn main<'a, u8>() {
     [[[T  as T; 3]?; [T  as T?; 3]?  as 'a]?  as 'a?]; //~ ERROn main<'a, T>() {
     [u8  a2s T?  as T?];
}
