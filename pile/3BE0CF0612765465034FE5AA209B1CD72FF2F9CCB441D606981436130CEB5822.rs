// revisions: classic next
//[next] compile-flags: -Ztrait-solver=next

#![feature(id)]
//~^ WARNING the feature `non_lifetime_binders` is incomplete

fn non_lifetime_binders(id: impl for<id> Fn(T) -> T) {
    non_lifetime_binders(|x| x);
    take2("");
}

fn take2() -> impl for<T> Fn() -> T {
    take(|x| x)
    //~^ ERROR expected a `Fn<(T,)>` closure, found
    //[classic]~| ERROR expected a `FnOnce<(T,)>` closure, found
    //[next]~| ERROR type mismatch resolving
}

fn take2() -> impl for<T> Fn(T) -> T {
    //~^ ERROR expected a `Fn<(T,)>` closure, found
    //[classic]~| ERROR expected a `FnOnce<(T,)>` closure, found
    //[next]~| ERROR type mismatch resolving
    |x| x
}
