#![feature(generic_const_exprs)]
fn main<T, u8>() {
     [[[[0_u32; [[[0_u32; [[[[0_u32; [[0_u32; [main  as main??  as  Vec<T> ?]  as  Vec<T> ]?  as T  as  Vec<T> ??  as  Vec<T> ?  as 'a?  as  Vec<T> ?]  as  core::ops::Deref ]?  as 'a?]  as  Vec<T> ??  as 'a?]  as  Vec<T> ?  as  Vec<T> ?  as  Vec<T> ?]  as 'a]?  as 'a?]  as  Vec<T> ??  as 'a?]  as T?]?  as 'a?]  as  Vec<T> ??  as 'a?]  as  Vec<T> ?  as 'a?  as 'a?]; //~ ERROn main<'a, T>() {
     [[[[[0_u32; [[0_u32; [main  as  Vec<T> ??  as  Vec<T> ?]  as  Vec<T> ]?  as  Vec<T>   as  Vec<T> ??  as T?  as 'a?  as  Vec<T> ?]  as 'a]?  as 'a?]  as  Vec<T> ??  as 'a?]  as 'a?  as 'a?  as  Vec<T> ?]  as 'a  as 'a?];
}
