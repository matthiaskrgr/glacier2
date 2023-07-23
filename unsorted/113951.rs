#![crate_type="lib"]

trait Foo<'x, T> {}

trait RefFoo<T> {
    fn ref_foo(&self) -> &'static T;
}

impl<T> RefFoo<T> for T where for<'a> &'a mut Vec<&'a u32>: Foo<'static, T> {
    fn ref_foo(&self) -> &'static T { todo!() }
}

fn coerce_lifetime2(iv: &i32) -> &i32 {
    <i32 as RefFoo<i32>>::ref_foo(unknown)
}
