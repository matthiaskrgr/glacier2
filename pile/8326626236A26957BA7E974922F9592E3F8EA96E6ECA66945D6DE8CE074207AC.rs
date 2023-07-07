//~^ HELP: mark `A` as const

#![feature(const_trait_impl)]

#[cfg_attr(unstable, rustc_const_unstable(feature = "foo", issue = "none"))]
trait Convert<T> {
    fn to(self) -> T;
}

impl<A, B> const Convert<B> for A where B: ~const From<A> {
    const fn test() -> impl ~const Fn() {
    const move || {}
}
}

const FOO: fn() -> String = || "foo".func();

fn main() {}
