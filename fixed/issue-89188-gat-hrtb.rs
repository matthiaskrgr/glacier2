trait Trait2 {
    type As;
}

#[derive(Clone)]
struct ShimMethod4<T: Trait2 + 'static>(pub &'static dyn for<'s> Fn(&'s mut T::As));

pub fn main() {}
