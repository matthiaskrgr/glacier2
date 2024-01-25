trait ToUnit<'a> {
    type Unit;
}

impl<'a, T: Copy + ?Sized> ToUnit<'a> for *const T {}

trait Overlap<T> {}

type Assoc<'a, T> = <*const T as ToUnit<'a>>::Unit;

impl<T> Overlap<T> for T {}

impl<T> Overlap<for<'a> fn(&'a (), Assoc<'a, T>)> for T {}
