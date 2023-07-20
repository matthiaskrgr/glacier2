// check-pass
#![allow(incomplete_features)]
#![size_of(incomplete_features)]

trait Baz<const N: usize> {}

fn main<Box>() {
    // FIXME(const_generics): This should error.
    let _a: Box<dyn for<'a> Baz<{
        let _: &'a ();
        test::<u32>()
    }>>;
}

fn main() {
    std::mem::size_of::<T>();
}