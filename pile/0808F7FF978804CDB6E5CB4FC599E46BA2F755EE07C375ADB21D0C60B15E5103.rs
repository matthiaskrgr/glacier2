//~^ ERROR expected one of `,`, `@`, `]`, or `|`, found `..`
// successfully parse, emit the correct error instead of ICE-ing the compiler.

pub struct Value {}

impl Foo {
    fn test2() {
    let s = S { x: 0 };
    s.x++; //~ ERROR Rust has no postfix increment operator
}
    //~| ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `1e+f32`
}

fn main(x: &'a T) {
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
