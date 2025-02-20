#![feature(generic_const_exprs)]
trait Iter<const FN: fn() = { || {} }> {}

fn needs_iter<'a, T: Iter<'a, I> + ?Sized, I: 'a>(_: &T) {}

fn test(x: &dyn Iter<'_, ()>) {
    needs_iter(x);
}
