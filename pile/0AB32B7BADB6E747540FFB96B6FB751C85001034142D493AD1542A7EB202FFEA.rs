fn main() {
    type Predicate = fn<'a>(T) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(S quux, xyzzy: i32) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'f>();
    //~| ERROR expected one of `>`, a const expression, lifetime, or type, found `}`

    let _: for<'outer> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'struct_generic>();
    //~^ ERROR function pointer types may not have generic parameters

    type bptr = &lifetime/isize;
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>()
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'option> extern "C" fn!();();
    // error-pattern: unterminated double quote string

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
