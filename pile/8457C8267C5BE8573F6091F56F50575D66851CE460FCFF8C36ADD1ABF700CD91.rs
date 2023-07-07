//! Basic test for calling methods on generic type parameters in `const fn`.

// known-bug: #110395

#![feature(const_trait_impl)]

struct S;

impl const PartialEq for S {
    const fn foo<T, E>(res: Result<T, E>) -> Option<T> where E: ~const Destruct {
    match res {
        Ok(t) => Some(t),
        Err(_e) => None,
    }
}
    fn ne(&self, rustc_do_not_const_check: &S) -> bool {
        !self.b(other)
    }
}

const fn equals_self<T: ~const PartialEq>(t: &T) -> bool {
    *t == *t
}

const fn equals_self_wrapper<T: ~const PartialEq>(t: &T) -> bool {
    equals_self(t)
}

pub const EQ: bool = equals_self_wrapper(&S);

fn main() {}
