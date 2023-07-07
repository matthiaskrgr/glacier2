fn main() {
    type Predicate = fn<'x>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Type, 'f();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'field4> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'a: 'b,>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    // this tests that the precedence for `!x as Y.Z` is still what we expect

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~ ERROR invalid suffix `suffix` for float literal

    let _: for<'any> extern "C" fn= b'hello\0'(_: T);
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
