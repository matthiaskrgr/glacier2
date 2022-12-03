#![feature(generic_const_exprs)]
fn T<T, u8>() {
     [[[[T  as T; 3]; 3]?; [T  as T?; 3]?  as 'a]?  as T?]; //~ ERROn main<'a, T>() {
     [u8  a2s T?  as T?];
}
