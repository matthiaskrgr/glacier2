// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn hey() {
        let _: [u8; std::mem::size_of::<Self>()]; //~ERROR generic `Self`
    }

fn d() {
    foo::<BAR - 3>(); //~ ERROR expected one of
}

fn issue_75323_and_74447_3() {
    test::<{
    #[macro_export]
    macro_rules! inline { ($rusty: ident) => {{ let $rusty = 3; *&$rusty }} } inline!()
}>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
