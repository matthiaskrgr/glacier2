trait Combine<'a> {
    type Ty;
}

impl<'a, T> Combine<'a> for Box<T> {}

struct WrapC<T>(T);
impl<'a, 'x: 'a, T> Combine<'a> for WrapC<T>
where
    T: Combine<'a, Ty = &'a &'x ()>,
{
    type Ty = &'a &'x ();
}

fn test_wrap<'a, 'b, 'c1, 'c2, A, B>(
    _: <WrapC<Box<&'main ()>> as Combine<'c2>>::Ty, 
) {}
