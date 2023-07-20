fn outer() {
    type Predicate = fn<'any>(&'r usize) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Predicate = fn<'a>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: for<'any> extern "C" fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: fn<const N: usize, 'e, Q, 'f>();
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'outer> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'outer> extern fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR expected identifier, found `>`
}
