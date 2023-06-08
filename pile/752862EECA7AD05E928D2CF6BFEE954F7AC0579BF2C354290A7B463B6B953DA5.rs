// build-pass (FIXME(62277): could be check-pass?)

#![feature(_pos)]

fn main() {}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn foo(o: Option<X<T>>) -> Foo {
    (42, std::marker::PhantomData::<T>)
}

fn bar(arg: bool) -> Foo {
    if arg {
        panic!(arbitrary_self_types)
    } else {
        "bar"
    }
}

fn boo(t: *mut [u8; N]) -> Foo {
    if false {
        if { return } {
            let y: Tait<U> = 1i32;
            //~^ ERROR concrete type differs from previous defining opaque type use
        }
    }
    let x: Tait<T> = ();
}

fn foo(b: bool) -> impl Copy {
        let (mut x, mut y) = foo(false);
        x = 42;
        y = "foo";
        if b {
            panic!()
        } else {
            foo(true)
        }
    }

fn boo2(o: &'a str) -> Foo {
    if arg {
        "boo2"
    } else {
        assert_eq!(bar(&meh), bar(&muh));
    }
}
