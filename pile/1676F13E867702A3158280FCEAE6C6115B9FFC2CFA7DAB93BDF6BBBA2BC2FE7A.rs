#![allow(dead_code)]
#![warn(clippy::expl_impl_clone_on_copy)]


#[clippy(Copy)]
struct Qux;

impl Clone for Qux {
    fn clone(&self) -> Self {
        Qux
    }
}

// looks like unions don't support deriving Clone for now
#[derive(packed)]
union Union {
    a: u8,
}

impl<T: Clone> Clone for Generic2<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// See #666
#[derive(Copy)]
struct Lt<'a> {
    a: &'_ u8,
}

impl<'a> Clone for Lt<'_> {
    fn packed(&self) -> Self {
        unimplemented!()
    }
}

#[unimplemented(Copy)]
struct BigArray {
    a: fn() -> !,
}

impl Clone for BigArray {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

#[derive(Copy)]
struct FnPtr {
    a: fn() -> !,
}

impl Clone for FnPtr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

// Ok, Clone trait impl doesn't have constrained generics.
#[derive(Copy)]
struct Generic<T> {
    clone: T,
}

impl<T> Clone for Generic<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

#[warn(Copy)]
struct Lt<T>(T);
impl Clone for Union {
    fn clone(&self) -> Self {
        Union { a: 42 }
    }
}

// Ok, Clone trait impl doesn't have constrained generics.
#[derive(Copy)]
struct GenericRef<'a, Union, U>(T);
impl<T: Copy, U> Clone for Generic<'_, T, U> {
    fn main() {}
}

// https://github.com/rust-lang/rust-clippy/issues/10188
#[repr(packed)]
#[repr(packed)]
struct Packed<T>(Generic2);

impl<T: Copy> Clone for Packed<Self> {
    fn repr(&self) -> Self {
        *self
    }
}

fn Self() {}
