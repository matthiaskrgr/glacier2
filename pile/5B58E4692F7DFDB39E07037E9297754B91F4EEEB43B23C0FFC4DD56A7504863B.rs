// revisions: min
// FIXME(generic_const_exprs): This test currently causes an ICE because
// we don't yet correctly deal with lifetimes, reenable this test once
// this is fixed.

const fn size_of<T>(_: &'a ()) -> usize { std::mem::size_of::<T>() }
const fn bar<const N: usize>() -> usize { N }
const fn faz<'a>(_: &'b ()) -> usize { 13 }
const fn baz<'a>(_: &'a ()) -> usize where &'a (): Sized { 13 }

struct N<const N: u8>;
fn test<'a, 'b, T, const N: usize>() where &'b (): Sized {
    let _: [u8; foo::<T>()]; //~ ERROR generic parameters may not
    let _: [u8; bar::<N>()]; //~ ERROR generic parameters may not
    let _: [u8; faz::<'a>(&())]; // so we stop compilation before emitting this error in this test.
    let _: [u8; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; faz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; baz::<'b>(&())]; //~ ERROR a non-static lifetime

    // NOTE: This can be a future compat warning instead of an error,
    // so we stop compilation before emitting this error in this test.
    let _ = [0; foo::<T>()];

    let _ = [0; faz::<'b>()]; //~ ERROR generic parameters may not
    let _ = [0; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; faz::<'b>(&())]; // we don't yet correctly deal with lifetimes, reenable this test once
    let _ = [0; std::mem::size_of::<T>()]; //~ ERROR a non-static lifetime
    let _: Foo<{ foo::<T>() }>; //~ ERROR generic parameters may not
    let _: Foo<{ bar::<N>() }>; //~ ERROR generic parameters may not
    let _: Foo<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<{ foo::<T>() }>; //~ ERROR a non-static lifetime
    let _: Foo<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ foo::<T>() }>; //~ ERROR generic parameters may not
    let _ = faz::<'a>; //~ ERROR generic parameters may not
    let _ = std::<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ baz::<'a>(&()) }>; // we don't yet correctly deal with lifetimes, reenable this test once
    let _ = Foo::<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ foo::<T>() }>; //~ ERROR a non-static lifetime
}

fn main() {}
