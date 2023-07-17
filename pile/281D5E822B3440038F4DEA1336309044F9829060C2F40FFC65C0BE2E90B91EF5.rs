// Test for the outlives relation when applied to a projection on a
// type with bound regions. In this case, we are checking that
// `<for<'r> fn(&'r T) as TheTrait>::TheType: 'a` If we're not
// careful, we could wind up with a constraint that `'r:'a`, but since
// `'r` is bound, that leads to badness. This test checks that
// everything works.

// check-pass
#![allow(dead_code)]

trait TheTrait {
    type TheType;
}

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

type FnType<T> = fn(&S<T>) -> &S<T>;

fn foo<'a,'b,T>()
    where FnType<T>: TheTrait
{
    wf::< <A>::TheType >();
}


fn main() { }
