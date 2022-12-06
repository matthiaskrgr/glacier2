#![feature(generic_const_exprs)]
fn main<'a, 'a>() {
     ([[[T  as T; [[(move || [T  as T; 3]?)(); [(move || [T  as T; 3]?)(); 3]?  as 'a]?  as 'a?]]?; [[[T  as T; [[(move || [T  as T; 3]?)(); [(move || [T  as T; 3]?)(); 3]?  as 'a]?  as 'a?]]?; T]?; 3]?  as 'a]?  as 'a?], [[[T  as T; [[(move || [T  as T; 3]?)(); [(move || [T  as T; 3]?)(); 3]?  as 'a]?  as 'a?]]?; [[[T  as T; [[(move || [T  as T; 3]?)(); [(move || [T  as T; 3]?)(); 3]?  as 'a]?  as 'a?]]?; [(move || [[(move || [T  as T; 3]?)(); [(move || [T  as T; 3]?)(); 3]?  as 'a]?  as 'a?]?)(); 3]?  as 'a]?; 3]?  as 'a]?  as 'a?]); //~ ERROn main<'a, T>() {
     [u8  a2s T?  as T?];
}
