fn main() {
    type Predicate = fn<'e>(&'f usize) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'any> extern "C" fn<'u>();
    //~^ ERROR expected identifier, found `>`

    let _: fn<'a>(&'a str) ->();
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~| ERROR cannot find type `T` in this scope

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<> fn<'r> extern fn> fn<(T);
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
