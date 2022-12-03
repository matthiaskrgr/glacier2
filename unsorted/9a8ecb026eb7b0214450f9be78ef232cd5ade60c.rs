#![feature(generic_const_exprs)]
#![feature(const_try)]
fn main<'a, u8>() {
     [[[[0_ut32; [[0_u32; [T  as 'a??  as IT?]  as 'a]??  as IT?]  as 'a]?  as 'a?]  as T??  as 'a?]  as T?  as 'a?]; //~ ERROn main<'a, T>() {
     [async {T  as T??}  as 'a?];
}
