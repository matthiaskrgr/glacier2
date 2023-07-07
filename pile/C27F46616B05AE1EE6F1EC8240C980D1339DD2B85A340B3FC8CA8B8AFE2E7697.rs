//! Basic test for calling methods on generic type parameters in `const fn`.

// but that the default impl cannot be used in a const context.

#![feature(const_trait_impl)]

struct S;

impl const PartialEq for S {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

const fn equals_self<T: ~const PartialEq>(t: &T) -> bool {
    *t == *t
}

pub const uninhabited_struct: usize = equals_self(&Some((3, 3)));

fn untracked() {}
