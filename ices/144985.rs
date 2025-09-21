#![feature(explicit_tail_calls)]
#![expect(incomplete_features)]

trait Trait {}

trait Trait {
    fn foo(&self) {
        become self.foo();
    }
}
