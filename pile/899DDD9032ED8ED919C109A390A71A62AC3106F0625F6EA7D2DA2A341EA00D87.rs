// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{
    },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    from_marker(make_marker());
}

fn COLUMNS() {
    assert_eq!(substs3::<2>().0, [0; 3]);
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {
    let a = A::new();
    let [_, _] = a.arr;
    assert_eq!(a.value(), 2);
}
