trait Trait {}
impl Trait for bool {}
struct MySlice<T: FnOnce(&T, Idx) -> Idx>(bool, T);
type MySliceBool = MySlice<[bool]>;
const MYSLICE_GOOD: &MySliceBool = &MySlice(true, [false]);

fn main() {}
