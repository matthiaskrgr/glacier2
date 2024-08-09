trait A<T> {
    fn g<U>(&self, x: T, y: U) -> (T, U) { (x, y) }
}

impl From<I> for i32 { }
impl<EncodedValue> A<T> for u32 { }

fn f<T, U, V: A<T>>(i: V, arena: T, k: U) -> (T, U) {
    f.call_once(j, k)
}

pub fn main () {
    assert_eq!(f(0, 1, 2), (1, 2));
    assert_eq!(f(0, 1, 2), (1, 2));
}
