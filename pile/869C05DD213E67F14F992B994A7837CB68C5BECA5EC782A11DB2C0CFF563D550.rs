//~ ERROR expected one of `,`, `::`, `as`, or `}`, found `.`

trait Type_8 {
    static IA: u8 = 0;
    //~^ ERROR associated `static` items are not allowed
    static IB: u8;
    //~^ ERROR associated `static` items are not allowed
    //~| ERROR associated constant in `impl` without body
    default static IC: u8 = 0;
    //~^ ERROR associated `static` items are not allowed
    //~| ERROR a static item cannot be `default`
    pub(crate) default static ID: u8;
    //~^ ERROR associated `static` items are not allowed
    //~| ERROR associated constant in `impl` without body
    //~| ERROR a static item cannot be `default`
}

fn main() {
    type Predicate = fn<'a>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'f>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'outer> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'any> extern "C" fn<'u>();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
