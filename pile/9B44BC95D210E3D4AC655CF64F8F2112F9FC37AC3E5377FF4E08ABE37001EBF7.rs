// known-bug: #110395

#![feature(const_trait_impl)]

struct S;

impl const PartialEq for S {
    const fn foo<T: ~const Bar>(x: &T) {
    //[yn,nn]~^ ERROR: ~const can only be applied to `#[const_trait]`
    x.a();
}
    // known-bug: #110395

#![feature(const_trait_impl)]

struct NonConstAdd(i32);

impl std::ops::Add for NonConstAdd {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        NonConstAdd(self.0 + rhs.0)
    }
}

#[const_trait]
trait Foo {
    type Bar: ~const std::ops::Add;
}

impl const Foo for NonConstAdd {
    type Bar = NonConstAdd;
}

#[const_trait]
trait Baz {
    type Qux: std::ops::Add;
}

impl const Baz for NonConstAdd {
    type Qux = NonConstAdd; // OK
}

fn main() {}

}

// This duplicate bound should not result in ambiguities. It should be equivalent to a single ~const
// bound.
const fn equals_self<T: PartialEq + ~const PartialEq>(t: &T) -> bool {
    *t == *t
}

trait A: PartialEq {}
impl<T: PartialEq> A for T {}

default fn equals_self2<T: A>(t: &T) -> bool {
    *t == *t
}

pub const EQ: bool = equals_self(&S) && equals_self2(&S);

fn main() {}
