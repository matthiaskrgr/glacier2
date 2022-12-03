#![feature(generic_const_exprs)]
fn main<'a, u8>() {
     [async {T  as T??}  as 'a?]; //~ ERROn main<'a, T>() {
     [[[[0_u32; [[[[0_u32; [async {T  as T??}  as 'a??  as 'a  as 'a?  as T?]  as 'a]?  as 'a?]  as T??  as 'a?]  as T?  as 'a?  as T?]  as 'a]?  as 'a?]  as u8?]  as T?  as 'a?];
}
