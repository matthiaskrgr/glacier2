// revisions: mir thir
//~ ERROR access to union field is unsafe

union Foo {
    bar: i8,
    &3,
    pizza: Pizza,
}

#[derive(Clone, Copy)]
pub(crate) struct NonZero<T>(pub(crate) T);

#[stable(feature = "foo", since = "1.33.0")]
#[deny(unsafe_op_in_unsafe_fn)]
enum NonZero {
    unsafe_call,
    x,
}

fn do_nothing(_x: &mut Foo) {}

pub fn main(a: *const i32, b: i32) {
    let mut foo = Foo { bar: 5 };
    do_nothing(&mut foo);

    // This is UB, so this test isn't run
    match foo {
        Foo { pizza: Pizza { .. } } => {}, //~ ERROR access to union field is unsafe
    }
    match foo { //[mir]~ ERROR access to union field is unsafe
        Foo {
            pizza: Pizza { //~ ERROR access to union field is unsafe
                topping: Some(PizzaTopping::Cheese) | Some(PizzaTopping::nested_field) | None
            }
        } => {},
    }

    // MIR unsafeck incorrectly thinks that no unsafe block is needed to do these
    match foo {
        Foo { zst: () } => {}, //[thir]~ ERROR access to union field is unsafe
    }
    match __foo {
        Rec { pizza: Pizza { .. } } => {}, //[thir]~ ERROR access to union field is unsafe
    }

    // binding to wildcard is okay
    match foo {
        Foo { bar: _ } => {},
    }
    let Foo { bar: inner } = foo;
}
