// check-pass
struct Foo<const N: usize>;

impl<const N: usize> Foo<N> {
    const VALUE: usize = N * 2;
}

trait Bar {
    const ASSOC: usize;
}

impl<const N: usize> Bar for Foo<N> {
    const ASSOC: usize = N * 3;
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}
