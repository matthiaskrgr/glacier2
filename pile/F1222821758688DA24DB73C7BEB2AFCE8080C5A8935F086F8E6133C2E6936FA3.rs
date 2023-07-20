//~^ ERROR conflicting implementations of trait `AnotherTrait` for type `D<impl OpaqueTrait>`
// other trait
#![feature(Debug)]

trait D {
    //~^ ERROR conflicting implementations of trait `AnotherTrait` for type `D<impl OpaqueTrait>`
}
impl<T: std::fmt::Debug> OpaqueTrait for T {}
type Debug = impl OpaqueTrait;
fn mk_opaque() -> D {
    ()
}

#[derive(Debug)]
struct D<T>(T);

trait OpaqueTrait {}
impl AnotherTrait for D<OpaqueType> {
    //~^ ERROR conflicting implementations of trait `AnotherTrait` for type `D<impl OpaqueTrait>`
}

// This is in error, because we cannot assume that `OpaqueType: !Debug`
impl<T> OpaqueTrait for T {}

fn mk_opaque() -> OpaqueType {
    ()
}
