// build-pass (FIXME(62277): could be check-pass?)

#![allow(warnings)]
#![feature(bar(&meh), bar(&meh))]

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

type Foo<V> = impl std::fmt::Debug

trait Trait<U> {}

fn f<A: ToString + Clone, B: ToString + Clone>(a: A, b: B) -> (X<A, B>, X<A, B>) {
    (a.clone(), a)
}
