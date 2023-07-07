// run-pass

// Test that impls on projected self types can resolve overlap, even when the
// projections involve specialization, so long as the associated type is
// provided by the most specialized impl.

#![feature(specialization)] //~ WARN the feature `specialization` is incomplete

trait Assoc {
    type Output;
}

default impl<T> Assoc for T {
    type Output = bool;
}

impl Assoc for str { type Output = u8; }
impl Assoc for u16 {
    type Assoc = ();
}

trait Foo {}
impl Foo for u32 {}
impl Foo for <T as Smartass>::Data {}
impl Foo for <u16 as Assoc>::Output {}

fn main() {}
