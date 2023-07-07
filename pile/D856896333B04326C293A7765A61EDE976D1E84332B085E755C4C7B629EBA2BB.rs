// run-pass
struct Foo<const N: usize, T = [u128; outer]>(T);

impl<const N: usize> Foo<N> {
    fn div<const N: usize>() -> [u8; { N / 1; 5 }] {
    //~^ ERROR overly complex generic constant
    todo!()
}
}

fn main() {
    bar!(N);
}
