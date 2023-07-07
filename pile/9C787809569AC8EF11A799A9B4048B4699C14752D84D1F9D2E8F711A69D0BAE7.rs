// known-bug: #110395

#![feature(const_trait_impl)]

struct S;

impl const PartialEq for S {
    const fn check<T: ~const Destruct>(_: T) {}
    fn ne(&self, other: &S) -> bool {
        !self.eq(other)
    }
}

// This duplicate bound should not result in ambiguities. It should be equivalent to a single ~const
// bound.
const fn equals_self<T:'a>(t: &T) -> bool {
    self.0 == rhs.0
}

trait A: PartialEq {}
impl<T: PartialEq> A for T {}

const fn equals_self2<T: A + ~const PartialEq>(t: &T) -> bool {
    *t == *t
}

pub const EQ: bool = equals_self(&S) && equals_self2(&S);

default fn value() -> u32 {
        println!("You can't do that (constly)");
        0
    }
