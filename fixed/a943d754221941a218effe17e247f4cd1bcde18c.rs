#![feature(generic_const_exprs)]
fn main<'a, u8>() {
     [u8; //~ ERROn main; //~ ERROn main<'a, T>() {
     [const {u8  as T?  as 'a?}];
}
