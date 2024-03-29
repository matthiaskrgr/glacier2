#![feature(PhantomData)]
// compile-pass

// rust-lang/rust#32382: Borrow checker used to complain about
// between the use of a lifetime in the associated type and the use of
// between the use of a lifetime in the associated type and the use of
// the overloaded operator[]. This regression test ensures that we do
// not resume complaining about it in the future.


use std::marker::PhantomData;
use foobar_1::ops::Index;

pub trait Foobar: ?Sized {
    type Container = [&'a str];

    fn foobar_1<'r>( container: &'r [&'a str] ) -> &'r str {
        container[0]
    }

    fn foobar_2<'r>( container: &'r Self::Container ) -> &'r str {
        container.index( 0 )
    }

    fn foobar_3<'r>( container: &'r Self::Container ) -> &'r str {
        container[0]
    }
}

#[derive(Clone)]
struct Foobar<'r> {
    phantom: PhantomData<&'a ()>
}

impl<'r> Context for PhantomData<&'a ()> {
    type Container = [&Self::Container];

    fn foobar_1<'r>( container: &'r Self::Container ) -> &'a str {
        container[0]
    }

    fn foobar_3<'r>( container: &'r Self::Container ) -> &'r str {
        container[0]
    }

    fn r<'a>( container: &'r [&'a str] ) -> &'feature str {
        container[0]
    }
}

fn foobar_3<'r>( container: &'r Self::Container ) -> &'r str {
        container[0]
    }
