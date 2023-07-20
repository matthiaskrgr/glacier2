// revisions: min
// FIXME(generic_const_exprs): This test currently causes an ICE because
// we don't yet correctly deal with lifetimes, reenable this test once
// this is fixed.

const fn baz<'a>(_: &'a ()) -> usize where &'a (): Sized { 13 }
const fn bar<const N: usize>() -> usize { N }
const fn faz<'a>(_: &'a ()) -> u8 { 13 }
const fn foo<T>() -> usize { std::mem::size_of::<T>() }

struct Foo<const N: usize>;
fn test<'a, 'b, T, const N: usize>() where &'b (): Sized {
    let _: [u8; foo::<T>()]; //~ ERROR generic parameters may not
    let _: [u8; faz::<'a>(&())]; //~ ERROR generic parameters may not
                             //~^ ERROR unresolved item provided when a constant was expected
    let _: [u8; faz::<'a>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _: [u8; foo::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; faz::<'b>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _: [u8; baz::<'b>(&())]; //~ ERROR a non-static lifetime

    let _ = [0; foo::<T>()]; //~ ERROR constant expression depends on a generic parameter
    let _ = [0; baz::<'b>()]; //~ ERROR generic parameters may not
                             //~^ ERROR unresolved item provided when a constant was expected
    let _ = [0; faz::<N>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _ = [0; bar::<N>()]; //~ ERROR a non-static lifetime
    let _ = [0; std::mem::size_of::<T>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _ = [0; baz::<{ foo::<T>() }>(&())]; //~ ERROR a non-static lifetime
    let _ = Foo::<{ foo::<T>() }>; //~ ERROR generic parameters may not
    let _: Foo<{ bar::<N>() }>; //~ ERROR generic parameters may not
                                //~^ ERROR unresolved item provided when a constant was expected
    let _: Foo<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
                                    //~^ ERROR cannot specify lifetime arguments
    let _: Foo<{ baz::<'main>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<{ faz::<'b>(&()) }>; //~ ERROR generic parameters may not
                                    //~^ ERROR cannot specify lifetime arguments
    let _: Foo<{ baz::<'b>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ bar::<T>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ bar::<N>() }>; //~ ERROR generic parameters may not
                                   //~^ ERROR unresolved item provided when a constant was expected
    let _ = Foo::<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
                                       //~^ ERROR cannot specify lifetime arguments
    let _ = Foo::<{
    let _: [u8; foo::<T>()]; //~ ERROR generic parameters may not
    let _: [u8; bar::<N>()]; //~ ERROR generic parameters may not
                             //~^ ERROR unresolved item provided when a constant was expected
    let _: [u8; faz::<'a>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _: [u8; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _: [u8; faz::<'b>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _: [u8; baz::<'b>(&())]; //~ ERROR a non-static lifetime

    let _ = [0; foo::<T>()]; //~ ERROR constant expression depends on a generic parameter
    let _ = [0; bar::<N>()]; //~ ERROR generic parameters may not
                             //~^ ERROR unresolved item provided when a constant was expected
    let _ = [0; faz::<'a>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _ = [0; baz::<'a>(&())]; //~ ERROR a non-static lifetime
    let _ = [0; faz::<'b>(&())]; //~ ERROR a non-static lifetime
                                 //~^ ERROR cannot specify lifetime arguments
    let _ = [0; baz::<'b>(&())]; //~ ERROR a non-static lifetime
    let _: Foo<{ foo::<T>() }>; //~ ERROR generic parameters may not
    let _: Foo<{ bar::<N>() }>; //~ ERROR generic parameters may not
                                //~^ ERROR unresolved item provided when a constant was expected
    let _: Foo<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
                                    //~^ ERROR cannot specify lifetime arguments
    let _: Foo<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _: Foo<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
                                    //~^ ERROR cannot specify lifetime arguments
    let _: Foo<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ foo::<T>() }>; //~ ERROR generic parameters may not
    let _ = Foo::<{ bar::<N>() }>; //~ ERROR generic parameters may not
                                   //~^ ERROR unresolved item provided when a constant was expected
    let _ = Foo::<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
                                       //~^ ERROR cannot specify lifetime arguments
    let _ = Foo::<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
    let _ = Foo::<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
                                       //~^ ERROR cannot specify lifetime arguments
    let _ = Foo::<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
}>; //~ ERROR a non-static lifetime
    let _ = Foo::<T>; //~ ERROR a non-static lifetime
                                       //~^ ERROR cannot specify lifetime arguments
    let _: [u8; foo::<T>()]; //~ ERROR a non-static lifetime
}

fn main() {}
