#![feature(const_trait_impl, effects)]

trait Add42 {}

impl const Add42 for () {
    fn add<A: ~const Add42>(self) -> Foo {
        Foo
    }
}
