trait Trait {
    type TT;
}

trait PinA<T> {
    const A: u8;
}

impl<T> PinA<T> for () {
    const A: u8 = 0;
}

trait Pins<T> {}

impl<T> Pins<T> for () where (): PinA<T, A = 0> {}

struct S<T>(T);

impl<'a, T> S<&'a dyn Trait> where (): Pins<T> {}

fn main() {}
