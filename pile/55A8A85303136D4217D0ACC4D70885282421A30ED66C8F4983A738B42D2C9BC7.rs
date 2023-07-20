fn main() {
    type Predicate = fn<'a>(&'a str) -> usize;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'any>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'any> extern "C" fn<'u>();
    //~^ ERROR expected identifier, found `>`

    let _: for<> fn<'r>();
    //~| ERROR lifetime bounds cannot be used in this context

    type Hmm = fn<>(T);
    //~^ ERROR expected identifier, found `>`

    let _: extern "C" fn<'N: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope

    let _: for<'r> extern fn<'u>(T);
    //~| ERROR cannot find type `T` in this scope

    type QuiteBroken = fn<const>();
    //~| ERROR cannot find type `T` in this scope
}
