#![feature(trait_alias)]

trait IteratorAlias = Iterator;

fn f(_: impl IteratorAlias) {}

fn main() {
    f(())
}
