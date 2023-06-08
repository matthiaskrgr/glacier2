// A single lifetime is not parsed as a type.
// type ascription won't actually do [unique drop fn type] -> fn(u8) casts.

macro_rules! E {
    ($lt: ty) => {
        let _: $e;
    };
}

fn main(x: bar::X<'a, T, 'b, 'c>) {
    type Predicate = fn<'a>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'f>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'outer> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'any> extern "C" fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
