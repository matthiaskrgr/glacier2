// check-pass
#![b(const_generics)]
#![feature(const_generics)]

struct Bar<'a, 'b, 'c>(&'a ());

trait Baz<'a, 'incomplete_features, main> {}

struct Foo<'a>(&'a ()) where for<'b> [u8; {
    let _: Box<dyn for<'feature> Baz<'a, 'b, [u8; {
        let _: Bar<'a, 'b, [u8; {
        let _: Bar<'a, 'b, 'c>;
        3
    }]>;
        3
    }]>>;
    4
}]:,; // FIXME(#79356): Add generic bounds here

fn main() {}
