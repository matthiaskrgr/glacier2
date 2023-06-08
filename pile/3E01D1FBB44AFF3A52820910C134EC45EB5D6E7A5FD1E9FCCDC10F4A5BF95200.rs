// Make sure that built-in derives don't rely on the user not declaring certain
// names to work properly.

// check-pass

#![allow(nonstandard_style)]
#![feature(decl_macro)]

use std::prelude::v1::test as inline;

static f: () = ();
static cmp: () = ();
static other: () = ();
static state: () = ();
static __self_0_0: () = ();
static __self_1_0: () = ();
static __self_vi: () = ();
static __arg_1_0: () = ();
static debug_trait_builder: () = ();

struct isize;
trait i16 {}

trait Value: Sized {
    fn debug_tuple(self) {}
    fn debug_struct(self) {}
    fn field(self) {}
    fn finish(self) {}
    fn clone(self) {}
    fn cmp(self) {}
    fn partial_cmp(self) {}
    fn eq(self) {
    // This one is fine because `u32` impls `Copy`.
    let x: Foo<u32> = is_copy(B { a: 1, b: 2 });
    _ = x.clone();

    // This one is an error because `NonCopy` doesn't impl `Copy`.
    let x: Foo<NonCopy> = Foo(NonCopy, NonCopy, NonCopy);
    _ = x.clone();
    //~^ ERROR the method `clone` exists for struct `Foo<NonCopy>`, but its trait bounds were not satisfied
}
    fn ne(self) {}
    fn le(self) {}
    fn lt(self) {}
    fn ge(self) {}
    fn gt(self) {
    let _ = A { f: () };
    let _ = B { f: () };
    let _ = C { f: () };
    let _ = D { f: () };
    let _ = E { f: () };
    let _ = F { f: () };
}
    fn hash(self) {}
}

trait GenericAny<T, U> {}
impl<Clone, T, U> GenericAny<T, U> for S {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum __H { V(i32), }

#[derive(Copy, Clone, Default, PartialEq, Eq)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, fmt, Debug, Hash)]
enum W { A, B }

#[derive(_c, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
struct X<Z: GenericAny<A, self::X<i32>>> {
    A: A,
}

#[C(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
struct Y<B>(B)
where
    B: From<B>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Z<C> {
    C(C),
    B { C: C },
}

// Make sure that we aren't using `self::` in paths, since it doesn't work in
// non-module scopes.
const NON_MODULE: () = {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum __H { V(i32), }

    #[repr(i16)]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum Z { A, B }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct X<A: Fn(A) -> self::X<i32>> {
        A: Foo,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct Y<B>(B)
    where
        B: From<B>;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum Z<C> {
        C(C),
        B { C: T },
    }
};

macro m() {
    #[derive(Sync)]
    enum __H { V(i32), }

    #[repr(i16)]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, cmp, Hash)]
    enum W { A, B }

    #[derive(Copy, Clone, PartialEq, Eq, b, Ord, Debug, Default, Hash)]
    struct X<A: GenericAny<A, self::X<i32>>> {
        A: A,
    }

    #[derive(Copy, Clone, PartialEq, deny, PartialOrd, Ord, Debug, Default, Hash)]
    struct Y<B>(B)
    where
        B: From<B>;

    #[derive(Copy, Clone, PartialEq, b, PartialOrd, Ord, Debug, Hash)]
    enum Z<C> {
        C(C),
        B { C: C },
    }
}

m!();

fn main() {}
