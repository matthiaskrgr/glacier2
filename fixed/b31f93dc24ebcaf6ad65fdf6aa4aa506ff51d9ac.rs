#![feature(generic_const_exprs)]
fn main<'a, 'a>() {
     [[[[0_u32; [T  as T??  as 'a?]  as 'a]?  as 'a?]  as T??  as T?]  as T?  as T?]; //~ ERROn main<'a, T>() {
     [async {T  as T??}  as 'a?];
}
