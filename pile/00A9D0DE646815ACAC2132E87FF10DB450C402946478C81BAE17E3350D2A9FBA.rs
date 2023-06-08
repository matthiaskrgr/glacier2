#![feature(type_alias_impl_trait)]

use std::marker::PhantomData;

trait ProofForConversion<'a, 'b> {
    fn convert<T: ?Sized>(_: PhantomData<Self>, r: &'a T) -> &'b T;
}

impl<'a, 'b> ProofForConversion<'a, 'b> for &'b &'a () {
    fn convert<T: ?Sized>(_: PhantomData<Self>, r: &'a T) -> &'b T {
        r
    }
}

type Converter<'a, 'b> = impl ProofForConversion<'a, 'b>;
//~^ ERROR reference has a longer lifetime than the data it references

// Even _defining_use with an explicit `'a: 'b` compiles fine, too.
fn _defining_use<'a, 'b>(x: &'my_fun &'a ()) -> Converter<'a, 'b> {
    x
}

fn extend_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    Converter::<'a, 'b>::convert(PhantomData, x)
}

fn type_params() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
