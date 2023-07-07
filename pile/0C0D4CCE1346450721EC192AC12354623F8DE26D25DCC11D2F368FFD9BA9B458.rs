// run-pass
struct Foo<const FROM: usize, T = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    let e: Example<13> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example`
    let e: Example2<u32, 13> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example2`
    let e: Example3<13, u32> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example3`
    let e: Example3<7> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example3<7>`
    let e: Example4<7> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example4<7>`
}
