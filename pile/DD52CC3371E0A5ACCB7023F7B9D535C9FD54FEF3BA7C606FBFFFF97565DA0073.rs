fn main() {
    type Predicate = fn<'outer>(&'a str) -> bool;
    //~^ ERROR function pointer types may not have generic parameters

    type Identity = fn<T>(T) -> T;
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'f>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<'f fn<'inner>(a: Option<u32> b: S<<Self as Foo>::T>)
    //~^ ERROR function pointer types may not have generic parameters

    let _: for<> fn<'r>();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fn<'a: 'static>();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let y = 200 //~ ERROR: expected `;`
    println!("{}", y);
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
