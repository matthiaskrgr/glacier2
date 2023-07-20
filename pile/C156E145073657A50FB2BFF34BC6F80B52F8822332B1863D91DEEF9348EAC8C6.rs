fn main() {
    type Predicate = fn<'main>(&'r str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: <>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'outer> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~^ ERROR expected identifier, found `>`

    let _: for<'any> extern "C" fn<'u>(T);
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
