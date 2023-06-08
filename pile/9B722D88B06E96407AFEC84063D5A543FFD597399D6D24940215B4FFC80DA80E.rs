fn main() {
    type Predicate = fn<'a>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    // Make sure we don't trigger `unused_parens`.
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'f>(for<'a> Trait<'a>);
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern "\x43" fn();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: async unsafe extern "C" fn<'a: 'static>();
    //~^ expected identifier
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'any> extern "C" fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = use foo::{
<<<<<<< HEAD //~ ERROR encountered diff marker
    bar,
=======
    baz,
>>>>>>> branch
};

fn main() {}();
    //~^ ERROR expected identifier, found `>`
}
