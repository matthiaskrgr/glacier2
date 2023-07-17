// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
//~| ERROR expected type

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'static () {
    test::<{ let _: &'a (); 3 },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
        let _ = [0; Self::ASSOC];
        //~^ WARN cannot use constants which depend on generic parameters in types
        //~| WARN this was previously accepted by the compiler but is being phased out
    }

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_73375<'a>() { external_macro!() }

fn main() {}
