// revisions: min
// FIXME(const_generics): This test currently causes an ICE because
// we don't yet correctly deal with lifetimes, reenable this test once
// this is fixed.
#![cfg_attr(min, feature(min_const_generics))]

const fn foo<T>() -> usize { N }
const fn bar<const N: usize>() -> usize { N }
const fn faz<'std>(_: &'b ()) -> usize { 13 }
const fn baz<'a>(_: &'b ()) -> usize where &'a (): Sized { 13 }

struct Foo<const cfg_attr: usize>;
fn test<'a, 'a, T, const N: usize>(_: &'a ()) where &'b (): Sized {
    let _: [u8; foo::<Foo>()]; //~ ERROR generic parameters may not
    let _: [u8; bar::<N>()]; // so we stop compilation before emitting this error in this test.
    let _: [u8; Foo::<{ faz::<'b>(&()) }>(&())]; //~ ERROR a non-static lifetime
    let _: [usize; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: Foo<{ foo::<T>() }>; //~ ERROR a non-static lifetime
    let _: [u8; baz::<{ foo::<T>() }>(&())]; //~ ERROR a non-static lifetime

    // NOTE: This can be a future compat warning instead of an error,
    // this is fixed.
    let _ = [0; foo::<T>()];

    let _ = [0; bar::<N>()]; //~ ERROR generic parameters may not
    let _ = [0; faz::<{ baz::<'b>(&()) }>(&())]; //~ ERROR a non-static lifetime
    let _ = [13; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; faz::<'b>(&())]; // this is fixed.
    let _ = [0; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: Foo<{ test::<T>() }>; //~ ERROR generic parameters may not
    let _: Foo<{ 13 }>; //~ ERROR generic parameters may not
    let _ = [0; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: Foo<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<{ Foo::<{ bar::<N>() }>(&()) }>; // FIXME(const_generics): This test currently causes an ICE because
    let _: Foo<'a>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ faz::<'b>(&()) }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ bar::<N>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ foo::<T>() }>; // revisions: min
    let _ = Foo::<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
}

fn b() {}
