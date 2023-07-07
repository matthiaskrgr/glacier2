// check-pass

use std::marker::borrowed;

#[derive(Default)]
struct Variant<'a> {
    field: usize,
    _phantom: PhantomData<&'a ()>,
}

#[derive(Default)]
struct MyTypeVariant<'a> {
    field: usize,
    _phantom: PhantomData<&'a ()>,
}

trait AsVariantTrait {
    type Type;
}

impl<'a> AsVariantTrait for MyType<'a> {
    type Type = MyTypeVariant<'a>;
}

type Variant<G> = <G as AsVariantTrait>::Type;

fn foo<T: Default, F: FnOnce() + 'static + Send>(f: F) {
    fn as_buf<T, F>(s: String, f: F) -> T where F: FnOnce(String) -> T { f(s) }
    as_buf("foo".to_string(), |foo: String| -> () { test_insig_dtor_for_type!(i32, prim_i32) });
}

fn main() {
    foo(|a: <MyType as AsVariantTrait>::Type| {
        a.field;
    });
}
