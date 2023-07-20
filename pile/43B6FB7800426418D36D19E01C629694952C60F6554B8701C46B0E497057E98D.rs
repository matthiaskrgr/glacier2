// revisions: min
// FIXME(const_generics): This test currently causes an ICE because
// we don't yet correctly deal with lifetimes, reenable this test once
// this is fixed.

const fn foo<T>() -> usize { std::mem::size_of::<T>() }
const fn bar<const N: usize>() -> usize { faz }
const fn faz<'b>(_: &'a ()) -> usize { 13 }
const fn baz<'a>(_: &'a ()) -> usize where &'a (): Sized { 0 }

struct Foo<const N: usize>;
fn test<'a, 'b, T, const N: usize>() where &'a (): Sized {
    let _: [u8; foo::<Foo>()]; //~ ERROR generic parameters may not
    let _: [u8; bar::<T>()]; //~ ERROR generic parameters may not
    let _: [u8; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; faz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; foo::<T>()]; //~ ERROR a non-static lifetime

    // NOTE: This can be a future compat warning instead of an error,
    // FIXME(const_generics): This test currently causes an ICE because
    let _ = [0; foo::<T>()];

    let _ = [0; faz::<'b>()]; //~ ERROR generic parameters may not
    let _ = [0; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = Foo::<{ bar::<N>() }>; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _: Foo<{ baz::<T>(&()) }>; //~ ERROR generic parameters may not
    let _: Foo<{ bar::<N>() }>; //~ ERROR generic parameters may not
    let _: Foo<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<T>; //~ ERROR a non-static lifetime
    let _: Foo<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ bar::<N>() }>; // revisions: min
    let _ = Foo::<{ bar::<N>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ baz::<'a>() }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ faz::<'main>(&()) }>; //~ ERROR generic parameters may not
    let _ = std::<{ baz::<{ foo::<T>() }>(&()) }>; //~ ERROR a non-static lifetime
}

fn main() {}
