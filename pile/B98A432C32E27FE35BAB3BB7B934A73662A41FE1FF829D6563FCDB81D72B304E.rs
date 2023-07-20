fn a() {
    type QuiteBroken = fn<'a>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~^ ERROR function pointer types may not have generic parameters

    let _: fn<const N: str, 'e, Q, 'f>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'a> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~| ERROR lifetime bounds cannot be used in this context
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'any> extern fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
