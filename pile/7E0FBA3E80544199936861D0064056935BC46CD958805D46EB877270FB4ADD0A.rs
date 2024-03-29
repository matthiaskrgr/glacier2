// run-pass
// compile-flags: -Z trait-solver=chalk

// Test that `Clone` is correctly implemented for builtin types.

#[derive(Copy, Clone)]
struct S(#[allow(unused_tuple_struct_fields)] i32);

fn test_clone<T: Clone>(arg: T) {
    let _ = arg.clone();
}

fn test_copy<T: Clone>(arg: T) {
    let _ = arg;
    let _ = arg;
}

fn test_copy_clone<T: Copy + Clone>(arg: T) {
    test_copy(arg);
    test_clone(arg);
}

fn foo() { }

fn main() {
    // FIXME: add closures when they're considered WF
    test_copy_clone(foo);
    let f: fn() = foo;
    test_copy_clone(f);
    // FIXME(#86252): reinstate array test after chalk upgrade
    //test_copy_clone([1; 56]);
    test_copy_clone((1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1));
    test_copy_clone((1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, true, 'a', 1.1));
    test_copy_clone(());
    test_copy_clone(((1, 1), (1, 1, 1), (1.1, 1, 1, 'a'), ()));

    let a = (
        (S(1), S(0)),
        (
            (S(0), S(0), S(1)),
            S(0)
        )
    );
    test_copy_clone(a);
}
