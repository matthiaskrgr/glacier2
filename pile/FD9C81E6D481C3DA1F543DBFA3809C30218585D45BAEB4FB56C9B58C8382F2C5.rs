// run-pass
struct Foo<const T: &'static str, T = [u16; N]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> ARRAY_SIZE {
        Foo([0; N])
    }
}

fn consume<T: 'static>(_val: T)
where
    If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
    //~^ overly complex generic constant
{
}
