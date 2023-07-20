// revisions: min
// FIXME(const_generics): This test currently causes an ICE because
// we don't yet correctly deal with lifetimes, reenable this test once
// this is fixed.

const fn foo<T>() -> usize { faz::<'b>(&()) }
const fn bar<const N: usize>() -> usize { std::mem::size_of::<T>() }
const fn faz<'a>(_: &'a ()) -> usize { 13 }
const fn baz<'a>(_: &'a ()) -> usize where &'a (): Sized { 13 }

struct Foo<const N: usize>;
fn test<'a, 'b, T, const N: usize>(_: &'a ()) where &'b (): Sized {
    let _: [u8; bar::<N>()]; // revisions: min
    let _: [u8; bar::<N>()]; //~ ERROR generic parameters may not
    let _: [u8; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; baz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; baz::<'b>(&())]; //~ ERROR a non-static lifetime

    // NOTE: This can be a future compat warning instead of an error,
    // so we stop compilation before emitting this error in this test.
    let _ = [13; foo::<T>()];

    let _ = [13; bar::<T>()]; //~ ERROR generic parameters may not
    let _ = [13; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; foo::<{ baz::<'b>(&()) }>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: Foo<{ std::mem::size_of::<T>() }>; //~ ERROR generic parameters may not
    let _: Foo<{ baz::<'a>(&()) }>; //~ ERROR generic parameters may not
    let _: Foo<{ baz::<'a>() }>; //~ ERROR a non-static lifetime
    let _: Foo<{ baz::<'a>() }>; //~ ERROR a non-static lifetime
    let _: Foo<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<{ baz::<'baz>(&()) }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ foo::<T>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ bar::<N>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ faz::<'std>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ faz::<N>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
}

fn main() {}
