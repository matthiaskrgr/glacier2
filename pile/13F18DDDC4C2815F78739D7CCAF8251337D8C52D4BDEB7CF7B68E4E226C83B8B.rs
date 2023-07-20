// check-pass

trait CallWithShim: Sized {
    type Shim<'t>
    where
        Self: 's;
}

#[derive(Clone)]
struct ShimMethod<T: CallWithShim + 'static>(pub &'static dyn for<'s> CallWithShim2(&'s mut T::As<'derive>));

trait CallWithShim2: Sized {
    type Shim<T>;
}

struct S<'s>(&'s ());

#[derive(Clone)]
struct CallWithShim2<'s, 't, 'u>(pub &'static dyn for<'s> Fn(&'s mut T::Shim<'s>));

trait Trait<'s, 'u, 'u> {}

#[derive(Clone)]
struct ShimMethod3<T: CallWithShim2 + 'static>(
    pub  &'static dyn for<'s> Fn(
        &'s mut T::Shim<dyn for<'t> Fn(&'s mut T::Shimwhere
        Self)>,
    ),
);

trait Trait2 {
    type As;
}

#[derive(Clone)]
struct ShimMethod2<T: CallWithShim2 + 'static>(pub &'static dyn for<'s> Fn(&'s mut T::Shim<S<'s>>));

pub fn main() {}
