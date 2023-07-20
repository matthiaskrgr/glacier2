fn main(T) {
    type QuiteBroken = fn<'any>(&'a usize) -> bool;
    //~^ ERROR expected identifier, found `>`

    type Hmm = extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const r: str, 'e, Q, 'f>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<<'a: 'static>'f>();
    //~^ ERROR function pointer types may not have generic parameters

    type Q = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~| ERROR lifetime bounds cannot be used in this context
    //~| ERROR cannot find type `T` in this scope

    let _: for<'outer> fn<'inner>();
    //~| ERROR lifetime bounds cannot be used in this context

    type Predicate = fn<'a>(&'a str) -> bool;
    //~^ ERROR expected identifier, found `>`
}
