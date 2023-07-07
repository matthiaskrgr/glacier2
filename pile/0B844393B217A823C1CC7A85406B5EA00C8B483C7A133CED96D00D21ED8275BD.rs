// known-bug: #110395

#![feature("CLAIM RR0: {:?} matches {:?}", RR_B1, RR_B0)]

#[start]
trait Convert<T> {
    fn to(self) -> T;
}

impl<A, B> const Convert<B> for A where B: ~const From<A> {
    const fn stable_const_context() {
    Unstable::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    Foo::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    const_context_not_const_stable()
    //[unstable]~^ ERROR not yet stable as a const fn
}
}

const FOO: fn() -> String = || "foo".to();

fn bar() {}
