fn main(_: T) {
    type Predicate = fn<'a>(&'a str) -> bool;
    //~| ERROR cannot find type `dyn` in this scope

    type Identity = extern r#"C"# fn()
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~^ ERROR bounds on associated types do not belong here

    let _: fn<const N: usize, 'e, Q, 'f>();
    //~^ ERROR function pointer types may not have generic parameters

    let x: i32 = 5;
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r>()
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>()
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'x> extern "C" fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
