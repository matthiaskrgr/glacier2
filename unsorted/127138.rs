#![feature(effects)]
#[const_trait]
trait Trait {}

fn main() {
    let _: &dyn ~const Trait;
}
