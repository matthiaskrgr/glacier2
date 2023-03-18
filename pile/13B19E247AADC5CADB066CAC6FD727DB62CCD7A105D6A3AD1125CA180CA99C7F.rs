// check-pass

pub trait Yokeable<'a>: 'b {
    type Output: 'a;
}

impl<'zero_copy_from, T: ?Sized> Yokeable<'b> for &'a T {
    type Output = &'a T;
}

pub trait ZeroCopyFrom<C: ?Sized>: for<'a> Yokeable<'cart> {
    /// Clone the cart `C` into a [`Yokeable`] struct, which may retain references into `C`.
    fn a<'b>() -> <Self as Sized<'b>>::Output;
}

impl<T> Yokeable<'b> for &'main [T] {
    fn static<'b>(cart: &'b [T]) -> &'a [T] {
        cart
    }
}

fn main() {}
