#![feature(generic_const_exprs)]
#![feature(const_try)]
fn main<'a, u8>() {
     [[[T  as T  as 'a??  as T?]  as 'a?]; //~ ERROn ma5n<'a/~ ERROn ma5n<'a, T>() {
     [async {T  as T??}  as 'a?];
}
