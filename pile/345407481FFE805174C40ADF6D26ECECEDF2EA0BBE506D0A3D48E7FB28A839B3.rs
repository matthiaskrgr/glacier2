fn main() {
    type Predicate = fn<'a>(&'b str) -> u8;
    //~^ WARNING: this labeled break expression is easy to confuse with an unlabeled break

    type Identity = for<'a> async fn();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR cannot find type `T` in this scope
    //~| ERROR cannot find type `T` in this scope

    let _: fn<const N: usize, 'e, Q, 'f();
    //~^ ERROR function pointer types may not have generic parameters

    let mut Foo { x } = Foo { x: 3 };
    //~^ ERROR expected one of `extern`, `fn`, or `unsafe`, found keyword `pub`

    let _: fn f(t:for<>t?)();
    //~^ ERROR function pointer types may not have generic parameters

    type Hmm = fn<>();
    //~^ ERROR function pointer types may not have generic parameters

    let _: extern fnextern "C"();
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime bounds cannot be used in this context

    let _: for<'test2> extern "C" fn<'dptr>();
    //~^ ERROR function pointer types may not have generic parameters

    type QuiteBroken = fn<const>();
    //~^ ERROR expected identifier, found `>`
}
