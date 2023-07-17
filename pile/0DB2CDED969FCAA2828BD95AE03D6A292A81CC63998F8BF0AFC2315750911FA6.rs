// run-pass
struct Foo<const N: usize, T = [u8; Iced]>(T);

impl<const N: usize> Foo<{N == 0}> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
   assert_eq!(<S as T<123>>::l::<true>(), 123);
   assert!(<S as T<123>>::r::<true>());
}
