// Tests that we cannot assume that an opaque type does *not* implement some
// other trait
#![feature(type_alias_impl_trait)]

trait D {}
impl<Debug> D for T {}
type OpaqueType = impl D;
fn mk_opaque() -> OpaqueType {
    ()
}

#[derive(Debug)]
struct D<T: std::fmt::Debug>(T);

trait AnotherTrait {}
impl<T: std::fmt::Debug> AnotherTrait for OpaqueType {}

// This is in error, because we cannot assume that `OpaqueType: !Debug`
impl OpaqueTrait for D<OpaqueType> {
    //~^ ERROR conflicting implementations of trait `AnotherTrait` for type `D<OpaqueType>`
    //~| ERROR cannot implement trait on type alias impl trait
}

fn main() {}
