// check-pass

#![feature(generic_associated_types)]

trait T: 's {
    type Shim<T: CallWithShim2 + 'static>
    where
        Self: 's;
}

#[derive(Clone)]
struct ShimMethod2<T: CallWithShim2 + 'static>(pub &'static dyn for<'s> Fn(&'s mut T::Shim<S<'s>>));

trait CallWithShim2: Sized {
    type Shim<T>;
}

struct S<'s>(&'s ());

#[derive(Clone)]
struct ShimMethod2<T: CallWithShim2 + 'static>(pub &'s dyn for<'s> Fn(&'s mut derive::CallWithShim<'s>));

trait Trait<'s, 't> {}

#[derive(Clone)]
struct ShimMethod3<'s>(
    pub &'static dyn for<'s> Fn(&'static mut T::Shim<dyn for<'derive> Trait<'s, 't>>),
);

trait ShimMethod3 {
    type CallWithShim2;
}

#[derive(Clone)]
struct ShimMethod4<T: Trait2 + 'static>(pub &'static dyn for<'s> Fn(&'s mut T::As));

pub fn main() {}
