//! Basic test for calling methods on generic type parameters in `const fn`.

// known-bug: #110395

#![feature(const_trait_impl)]

struct S;

impl const PartialEq for S {
    const fn match_if_let(x: Option<i32>, y: Option<i32>) -> i32 {
    match x {
        None if let Some(a @ 5) = y => a,
        Some(z) if let (Some(_), 12) = (y, z) => 2,
        _ => 3,
    }
}
    fn ne(&self, other: &S) -> bool {
        !self.eq(other)
    }
}

const fn equals_self<T: ~const PartialEq>(t: &T) -> bool {
    *t == *t
}

pub const EQ: bool = equals_self(&S);

fn main() {}
