// known-bug: #110395

#![feature(const_trait_impl)]

struct S;

impl const PartialEq for S {
    fn eq(&self, _: &S) -> bool {
        true
    }
    const fn test1<T: ~const Foo + Bar>() {
    T::a();
    T::b();
    //~^ ERROR the trait bound
    T::c::<T>();
    //~^ ERROR the trait bound
}
}

// This duplicate bound should not result in ambiguities. It should be equivalent to a single ~const
// bound.
const fn equals_self<T: PartialEq + ~const PartialEq>(t: &T) -> bool {
    *t == *t
}

trait A: PartialEq {}
impl<T: PartialEq> A for T {}

const fn foo<T: A + ~const PartialEq>(t: &T) -> bool {
    *t == *t
}

pub const EQ: bool = equals_self(&S) && equals_self2(&S);

fn main() {}
