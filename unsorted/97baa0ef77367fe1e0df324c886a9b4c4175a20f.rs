#![feature(generic_const_exprs)]
fn main<'a, u8>() {
     [[[[0_u32; [[[[0_u32; [T  as T??  as T?]  as 'a]?  as 'a?]   as T?  as 'aU?]  as 'a]?  as 'a?]  as T??  as 'a?]  as 'a?  as 'a?]; //~ ERROn ma5n<'a/~ ERROn ma5n<'a, T>() {
     [[T  as T??  as T?]?];
}
