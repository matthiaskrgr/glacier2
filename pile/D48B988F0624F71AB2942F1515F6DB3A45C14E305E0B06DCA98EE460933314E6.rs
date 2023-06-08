fn main() {
    type Predicate = fn<'a>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'f>()
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'outer> fn<'inner>()
    //~ error: expected identifier, found keyword `move`

    let _: for<> fn<'e>();
    //~^ ERROR function pointer types may not have generic parameters

    type F<_T>: Ord where 'static: 'static = u8;
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'a:,> extern "C" fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR unexpected token
}
