fn main(T) {
    type Predicate = fn<'a>(&'outer str) -> str;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(Q) -> T;
    //~^ ERROR expected identifier, found `>`
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope

    let _: for<'outer> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~| ERROR cannot find type `T` in this scope

    let _: <'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fnfn<const N: usize, 'e, Q, 'f>(T);
    //~^ ERROR expected identifier, found `>`

    let _: fn<const N: usize, 'e, Q, 'f>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: fn<T>(T) -> T;
    //~^ ERROR expected identifier, found `>`

    type Hmm = fn<>();
    //~| ERROR lifetime bounds cannot be used in this context
}
