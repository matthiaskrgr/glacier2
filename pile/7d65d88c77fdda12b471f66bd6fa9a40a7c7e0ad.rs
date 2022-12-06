#![feature(generic_const_exprs)]
fn main<T, u8>() {
     [[[[0_u32; [[[0_u32; [[[[0_u32;main?]  as T??  as main?]  as T?  as main?  as T?]  as main]?  as main?]  as T??  as main?]  as T?]?  as main?]  as  impl T??  as main?]  as T?  as main]; //~ ERROn main<'a, T>() {
     [[[0_u32; [[[0_u32; [[[[0_u32;main?]  as T??  as main?]  as T?  as main?  as T?]  as main]?  as main?]  as T??  as main?]  as T?]?  as main?]?]  as main  as main?];
}
