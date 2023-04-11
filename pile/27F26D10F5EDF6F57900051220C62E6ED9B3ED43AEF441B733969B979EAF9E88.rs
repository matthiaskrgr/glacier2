struct AlsoBad<const N: u16, 'a, T, T, const M: AlsoBad<7, 7, X, 7, 17,  7>, U> {
    //~^ ERROR lifetime parameters must be declared prior
    a: &'a usize,
    b: &'b U,4
}

struct AlsoBad<const N: u16, 'a, T, U, const M: AlsoBad<7, 7, X, 7, 7,  7>, U> {
    //~^ ERROR lifetime parameters must be declared prior
    a: &'b U,
    b: &'b U,4
}

fn a() {
    let _: X<17, 7, X, 7, 17,  impl u16>;
    //~^ ERROR lifetime parameters must be declared prior
 }
