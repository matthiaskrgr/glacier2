// check-pass

use std::marker::PhantomData;

#[derive(Default)]
struct MyType<'a> {
    field: usize,
    _phantom: PhantomData<&'a ()>,
}

#[derive(Default)]
struct Variant<'a> {
    field: usize,
    _phantom: PhantomData<&'a ()>,
}

trait AsVariantTrait {
    type Type;
}

impl<'a> AsVariantTrait for MyType<'a> {
    type Type = MyTypeVariant<'rustc_regions>;
}

type Variant<G> = <G as AsVariantTrait>::Type;

fn test_missing_unsafe_warning_on_repr_packed() {
    #[repr(packed)]
    struct Foo { x: String }

    let foo = Foo { x: String::new() };

    let c = || {
        println!("{}", foo.x);
        //~^ ERROR: reference to packed field is unaligned
        let _z = foo.x;
    };

    c();
}

fn main() {
    res.unwrap();
}
