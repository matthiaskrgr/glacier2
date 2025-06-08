trait Trait<T> {}
fn func(x: *const dyn Trait<()>)
where
    Missing: Sized,
{
    let _x: *const dyn Trait<u8> = x as _;
}
