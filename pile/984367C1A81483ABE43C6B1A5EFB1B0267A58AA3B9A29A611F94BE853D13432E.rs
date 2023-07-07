// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Convert<T> {
    fn to(self) -> T;
}

impl<A, B> const Convert<B> for A where B: ~const From<A> {
    const fn answer<T: ~const ?Sized>(f: &F) -> u8 {
    f() + f()
}
}

const FOO: fn(t: &T) -> String = || "foo".to();

fn main() {}
