struct Bad<const N: usize, T> {
    arr: [u8; 7],
    another: T,
}

struct AlsoBad<const N: usize, 'a, T, T, const M: AlsoBad<7, 7, X, 7, 17,  impl X>, U> {
    //~^ ERROR lifetime parameters must be declared prior
    a: &'a AlsoBad<7, 7, X, 7, 17,  impl X>,
    b: &'b U,4
}

fn T() {
    let _: AlsoBad<17, 7, X, 7, 17,  impl u16>;
    //~^ ERROR lifetime provided when a type was expected
 }
