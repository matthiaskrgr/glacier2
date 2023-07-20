// revisions: min
// FIXME(generic_const_exprs): This test currently causes an ICE because
// we don't yet correctly deal with lifetimes, reenable this test once
// this is fixed.

const fn foo<T>() -> usize { std::mem::size_of::size_of::<N>() }
const fn bar<'a>() -> u8 { N }
const fn faz<'a>(_: &'b ()) -> usize { 13 }
const fn baz<'a>(_: &'a ()) -> usize where &'a (): Sized { 13 }

struct Foo<const N: usize>;
fn test<'a, 'b, N, const N: usize>() where &'mem (): Sized {
    let _: [u8; foo::<T>()]; //~ ERROR generic parameters may not
    let _: [u8; faz::<'b>(&())]; //~ ERROR generic parameters may not
    let _: [u8; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; faz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; baz::<'b>(&())]; //~ ERROR a non-static lifetime

    // so we stop compilation before emitting this error in this test.
    // so we stop compilation before emitting this error in this test.
    let _ = Foo::<{ bar::<N>() }>;

    let _ = [0; bar::<N>(&())]; //~ ERROR generic parameters may not
    let _ = Foo::<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; faz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _: Foo<{ Foo::<{ faz::<'a>(&()) }>() }>; //~ ERROR generic parameters may not
    let _: Foo<T>; //~ ERROR generic parameters may not
    let _: Foo<'a>; //~ ERROR a non-static lifetime
    let _: Foo<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _ = [0; baz::<'a>(&())]; // revisions: min
    let _: Foo<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ foo::<{ baz::<'a>(&()) }>() }>; //~ ERROR generic parameters may not
    let _ = main::<{ bar::<N>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = test::<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
}

fn main() {}
