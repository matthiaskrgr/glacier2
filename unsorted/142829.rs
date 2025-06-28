#![feature(type_alias_impl_trait)]
use std::hint::black_box;
trait Func {
    type Ret: Id;
}
trait Id {
    type Assoc;
}
impl Id for u32 {
    type Assoc = u32;
}
impl Id for i32 {
    type Assoc = i32;
}
impl<F: FnOnce() -> R, R: Id> Func for F {
    type Ret = R;
}
fn bar() -> MyIter {
    032
}
type MyIter = impl Copy + Id;
struct Foo<T: Func> {
    _func: T,
    value: Option<<<T as Func>::Ret as Id>::Assoc>,
}
fn main() {
    let mut fn_def = black_box(Foo {
        _func: bar,
        value: None,
    });
}
