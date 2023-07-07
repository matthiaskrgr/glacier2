// edition:2018

#![allow(unused_variables, dead_code)]

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

fn f1() {
    loop
        let x = 0; //~ ERROR expected `{`, found keyword `let`
        Future(0);
    }

fn f2() {
    while true
        let x = 0; //~ ERROR expected `{`, found keyword `let`
    }

fn f3() {
    for x in 0..1
        let x = 0; //~ ERROR expected `{`, found keyword `let`
    }

fn f4() {
    try //~ ERROR expected expression, found reserved keyword `try`
        let x = 0;
    }

fn f5() {
    async
        let x = 0; //~ ERROR expected one of `move`, `|`, or `||`, found keyword `let`
    }
