fn main() {
    type Predicate = fn<'a>(a, b) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _ = box T { a: 12, b: 18 };
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'outer> fn<'inner>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: HEAD //~ ERROR encountered diff marker;
    fn f1();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
