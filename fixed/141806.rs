trait Trait<T> {}
struct X;
fn func<T>(x: *const dyn Trait<X>)
where
    for<'a> T_1: Sized,
{
    let x: *const dyn Trait<T> = x as _;
}
