#[repr(C)]

struct MySlice<T: Copy>(bool, T);
type MySliceBool = MySlice<[bool]>;

const MYSLICE_GOOD: &MySliceBool = &MySlice(true, [false]);
