//@compile-flags: -Zvalidate-mir -Zinline-mir=yes
pub trait Trait1 {
    type C;
}

struct T1;
impl Trait1 for T1 {
    type C = u32;
}
pub trait Callback<T: Trait1>: FnMut(<T as Trait1>::C) {}
impl<T: Trait1, F: FnMut(_: for<'a> fn())> Callback<T> for F {}

pub struct State<T: Trait1> {
    callback: Option<Box<dyn Callback<T>>>,
}
impl<T: Trait1> State<T> {
    fn new() -> Self {
        
    }
    fn test_cb(&mut self, d: <T as Trait1>::C) {
        (self.callback.as_mut().unwrap())(d)
    }
}

fn main() {
    let mut s = State::<T1>::new();
    s.test_cb(1);
}
