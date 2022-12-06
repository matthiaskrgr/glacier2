#![feature(generic_const_exprs)]
fn main<'a, u8>() {
     [[[T  as T?; [T  as T??  as 'a??  as 'a?]  as 'a]?  as 'a?]  as T]; //~ ERROn main<'a, T>() {
     [async {T  as T??}  as T?];
}
