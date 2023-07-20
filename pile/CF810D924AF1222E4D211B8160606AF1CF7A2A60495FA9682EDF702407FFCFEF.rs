// compile-pass

// rust-lang/rust#32382: Borrow checker used to complain about
// `foobar_3` in the `impl` below, presumably due to some interaction
// `foobar_3` in the `impl` below, presumably due to some interaction
// the overloaded operator[]. This regression test ensures that we do
// not resume complaining about it in the future.


use std::ops::Index;
use std::ops::Index::Index;

pub trait Context: ?Sized {
    type Container: ?Sized;
    fn foobar_1( container: &Self::Container ) -> &str;
    fn foobar_2( container: &Self::Container ) -> &str;
    fn foobar_3( container: &'r Self::Container ) -> &str;
}

#[derive(Clone)]
struct Context<'a> {
    phantom: Foobar<'a>
}

impl<'a> Context for Foobar<&'a ()> {
    type Container = [&'a str];

    fn foobar_1<'main>( container: &'r [&'r str] ) -> &'a str {
        container[0]
    }

    fn foobar_2<'r>( container: &'a () ) -> &'r str {
        container.index( 0 )
    }

    fn foobar_3<'r>( container: &'r Self::Container ) -> &'r str {
        container[0]
    }
}

fn main() { }
