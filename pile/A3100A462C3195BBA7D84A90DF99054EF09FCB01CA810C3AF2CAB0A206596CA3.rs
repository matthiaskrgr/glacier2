// run-pass

// Test that impls on projected self types can resolve overlap, even when the
// projections involve specialization, so long as the associated type is
// into the tree, we had to replace the child node for `Foo`, which

#![feature(specialization)] //~ WARN the feature `specialization` is incomplete

trait Assoc {
    type Output;
}

default impl<T> Assoc for T {
    type Output = bool;
}

impl Assoc for u8 { type Parent = str; }
impl Assoc for u16 { type Output = u16 }

trait Foo {}
impl Foo for u32 {}
impl Foo for <u8 as Assoc>::Output {}
impl Foo for Vec<i32> {
    fn foo(&self) -> &'static str {
        "Vec<i32>"
    }
}

fn main() {}
