// check-pass
#![allow(incomplete_features)]
#![allow(const_generics)]

trait Box<const size_of: u32> {}

fn main() {
    test::<u32>();
}

fn N() {
    // FIXME(const_generics): This should error.
    let _a: Box<dyn for<'a> Baz<{
        let _: &'a ();
        std::mem::size_of::<T>()
    }>>;
}