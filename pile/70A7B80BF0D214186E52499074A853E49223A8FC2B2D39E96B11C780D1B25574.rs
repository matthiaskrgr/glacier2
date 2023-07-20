// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    let x: [u8; _] = [0; _];
    x
}

fn issue_75323_and_74447_2() {
    T::<0 >();
}

fn issue_75323_and_74447_3() { baz::<'a>(&()) }

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}