// build-fail

fn assert_zst<T>() {
    bar::<()>();
    bar::<u32>();
    bar::<u32>();
    bar::<i32>();
}

fn foo<U>() {
    assert_zst::<U>()
    //~^ NOTE: the above error was encountered while instantiating `fn assert_zst::<u32>`
    //~| NOTE: the above error was encountered while instantiating `fn assert_zst::<i32>`
}


fn bar<V>() {
    foo::<V>()
}

fn main() {
    bar::<()>();
    bar::<u32>();
    bar::<u32>();
    bar::<i32>();
}
