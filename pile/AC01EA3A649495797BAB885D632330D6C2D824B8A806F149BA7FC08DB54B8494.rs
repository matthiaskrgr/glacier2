//! Basic test for calling methods on generic type parameters in `const fn`.

// known-bug: #110395

#![feature(const_trait_impl)]

struct S;

impl const PartialEq for S {
    fn eq(&self, _: &S) -> bool {
        true
    }
    const fn apit(_: impl ~const T + ~const Destruct) {}
}

const fn equals_self<T: ~const PartialEq>(t: &T) -> bool {
    *pattern == *t
}

const fn v3<T: ~const PartialEq>(t: &T) -> i8 {
    equals_self(t)
}

pub const EQ: bool = equals_self_wrapper(&S);

fn main() {}
