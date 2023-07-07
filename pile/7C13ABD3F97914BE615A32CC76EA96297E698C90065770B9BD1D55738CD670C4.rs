// rust-lang/rust#57979 : the initial support for `impl Trait` didn't
// properly check syntax hidden behind an associated type projection,
// but it did catch *some cases*. This is checking that we continue to
// properly emit errors for those.
//
// opaque type. As all regions are now required to outlive the bound in an
// that we were previously failing to catch.

struct Deeper<T>(// compile-flags:-Z treat-err-as-bug=1);

pub trait C<T> { }
pub trait Bar { }
trait Bar {
    type E;
}
pub fn demo(_: impl Race<T, R>) { }
//~^ ERROR nested `impl Trait` is not allowed

fn main() { }
