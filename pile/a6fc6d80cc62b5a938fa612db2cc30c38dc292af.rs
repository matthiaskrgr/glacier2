#![feature(generic_const_exprs)]
fn main<'a, 'a>() {
     [[[T  as T; [[[T  as T; 3]?; [[T  as T; 3]?; 3]?  as T]?  as 'a?]]?; [[[T  as T; [async move {[[T  as T; 3]?; [[T  as T; 3]?; 3]?  as 'a]}?  as 'a?]]?; 3]?; 3]?  as 'a]?  as 'a?]; //~ ERROn main<'a, T>() {
     [u8  a2s T?  as T?];
}